// MIT License
//
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate log;

extern crate tokio_tungstenite;
extern crate tokio;
extern crate futures_util;

extern crate serde_json;

use bson::Document;
use futures_util::{
    SinkExt, StreamExt,
};
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::mpsc::Receiver;
use zawgl_front::cypher::query_engine::CypherError;
use std::sync::Mutex;
use zawgl_front::tx_handler::request_handler::GraphRequestHandler;
use zawgl_front::tx_handler::handler::GraphTxHandler;
use parking_lot::ReentrantMutex;
use tokio_tungstenite::tungstenite::Message;
use std::cell::RefCell;
use std::sync::Arc;
use log::*;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use std::result::Result;
use crate::open_cypher_request_handler::handle_open_cypher_request;
use tokio::sync::oneshot::{self, Sender};
use tokio::task::JoinSet;
mod result;
mod open_cypher_request_handler;
use self::result::ServerError;
use zawgl_core::model::init::InitContext;

type ResponseMessage = (Document, Sender<Result<Document, CypherError>>);

async fn accept_connection(stream: TcpStream, msg_tx: UnboundedSender<ResponseMessage>) {
    if let Err(e) = handle_connection(stream, msg_tx).await {
        match e {
            ServerError::WebsocketError(te) => match te {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => error!("error processing connection: {}", err),
            },
            ServerError::ParsingError(err_msg) => error!("parsing error: {}", err_msg),
            ServerError::HeaderError => error!("wrong header"),
            ServerError::CypherTxError(err) => error!("tx error {}", err),
            ServerError::ConcurrencyError => error!("cocurrency error"),
        }
    }
}


async fn handle_connection(stream: TcpStream, msg_tx: UnboundedSender<ResponseMessage>) -> Result<(), ServerError> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let mut msg_fut = ws_receiver.next();
    loop {
        match msg_fut.await {
            Some(msg) => {
                let msg = msg.map_err(ServerError::WebsocketError)?;
                if msg.is_binary() {
                    let open_cypher_prefix = "!application/openCypher".as_bytes();
                    let data = msg.into_data();
                    if data.len() > open_cypher_prefix.len() &&  &data[..open_cypher_prefix.len()] == open_cypher_prefix {
                        let doc = Document::from_reader(&data[open_cypher_prefix.len()..]).map_err(|err| ServerError::ParsingError(err.to_string()))?;
                        debug!("incoming message {}", doc.to_string());
                        let (tx, rx) = oneshot::channel();
                        msg_tx.send((doc, tx)).map_err(|_| ServerError::ConcurrencyError)?;
                        let cypher_reply = rx.await.map_err(|_| ServerError::ConcurrencyError)?;
                        let mut response_data = Vec::new();
                        cypher_reply.map_err(ServerError::CypherTxError)?.to_writer(&mut response_data).map_err(|err| ServerError::ParsingError(err.to_string()))?;
                        let response = Message::Binary(response_data);
                        ws_sender.send(response).await.map_err(ServerError::WebsocketError)?;
                    } else {
                        return Err(ServerError::HeaderError);
                    }
                }
                else if msg.is_close() {
                    break;
                }
                msg_fut = ws_receiver.next(); // Receive next WebSocket message.
            }
            None => break, // WebSocket stream terminated.
        }
    }

    Ok(())
}

pub async fn run_server<F>(addr: &str, conf: InitContext, callback: F, mut rx_run: Receiver<bool>) -> JoinSet<()> where F : FnOnce() -> () {
        
        
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Websocket listening on: {}", addr);
    let (msg_tx, mut msg_rx) = tokio::sync::mpsc::unbounded_channel::<ResponseMessage>();
        
    let graph_request_handler = Arc::new(Mutex::new(GraphRequestHandler::new(conf)));
    let tx_ref = Arc::clone(&graph_request_handler);
    let mut set = JoinSet::new();
    set.spawn(async move {
        let tx_handler = Arc::new(ReentrantMutex::new(RefCell::new(GraphTxHandler::new())));
        while let Some((doc, sender)) = msg_rx.recv().await {
            let cypher_reply = handle_open_cypher_request(Arc::clone(&tx_handler), Arc::clone(&tx_ref), &doc);
            if let Err(_err) = sender.send(cypher_reply) {
                error!("sending reply");
                break;
            }
        }
    });
    let commit_ref = Arc::clone(&graph_request_handler);
    set.spawn(async move {
        while let Some(run) = rx_run.recv().await {
            commit_ref.lock().unwrap().commit();
            if !run {
                break;
            }
        }
    });
    callback();

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);
        tokio::spawn(accept_connection(stream, msg_tx.clone()));
    }
    
    set
}
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

use super::gremlin_state::{State, StateContext};
use super::match_vertex_state::MatchVertexState;
use super::set_property_state::SetPropertyState;
use zawgl_core::model::{Relationship, Status, Node};
use super::super::super::gremlin::*;
use super::gremlin_state::*;
use std::convert::TryFrom;

pub struct FromState {
    source: Source,
}

enum Source {
    Alias(String),
    VertexId(u64),
}

impl FromState {
    pub fn new(source: &GValueOrVertex) -> Option<Self> {
        match source {
            GValueOrVertex::Value(value) => {Some(FromState{source: Source::Alias(String::from(value.as_str()?))})}
            GValueOrVertex::Vertex(vertex) => {Some(FromState{source: Source::VertexId(u64::try_from(vertex.id.clone()).ok()?)})}
        }
        
    }
}
impl State for FromState {


    fn handle_step(&self, context: &mut StateContext) -> Result<(), GremlinStateError> {
        match &context.previous_step {
            GStep::AddE(label) => {
                let mut r = Relationship::new();
                r.set_labels(vec![label.clone()]);
                r.set_status(Status::Create);
                let pattern = context.patterns.last_mut().ok_or(GremlinStateError::WrongContext("missing pattern"))?;
                let target_id = context.node_index;
                let source_id = match &self.source {
                    Source::Alias(a) => {*context.node_aliases.get(a).ok_or(GremlinStateError::WrongContext("missing alias"))?}
                    Source::VertexId(vid) => {
                        let mut source = Node::new();
                        source.set_id(Some(*vid));
                        source.set_status(Status::Match);
                        pattern.add_node(source)
                    }
                };
                if let Some(tid) = target_id {
                    let rid = pattern.add_relationship(r, source_id, tid);
                    context.relationship_index = Some(rid);
                } else {
                    return Err(GremlinStateError::WrongContext("missing target id in in from state"))
                }
            }
            _ => {} 
        }
        Ok(())
    }

    fn create_state(&self, step: &GStep) -> Result<Box<dyn State>, GremlinStateError> {
        match step {
            GStep::V(vid) => {
                Ok(Box::new(MatchVertexState::new(vid)))
            }
            GStep::SetProperty(name, value) => {
                Ok(Box::new(SetPropertyState::new(name, value)))
            }
            GStep::Empty => {
                Ok(Box::new(EndState::new()))
            }
            _ => {
                Err(GremlinStateError::Invalid(step.clone()))
            }
        }
    }
}
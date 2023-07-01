// MIT License
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

use bson::Document;
use zawgl_client::Client;
use zawgl_client::parameters::*;
use cypher_tests::run_test;


#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_cypher_0() {
    run_test("test_cypher_requests_complete_graph", 8182, test_cypher_requests_complete_graph).await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_cypher_2() {
    run_test("first_test", 8183, test_cypher_requests).await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_cypher_3() {
    run_test("another_test", 8185, test_double_create_issue).await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_cypher_4() {
    run_test("create_path_test", 8184, test_create_path).await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_cypher_5() {
    run_test("test_mutliple_match", 8187, test_mutliple_match).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_cypher_6() {
    run_test("test_cypher_self_relationship", 8189, test_cypher_self_relationship).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_cypher_7() {
    run_test("test_cypher_self_relationship_2", 8190, test_cypher_self_relationship_2).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_cypher_8() {
    run_test("test_where_clause_on_ids", 8191, test_where_clause_on_ids).await;
}

async fn test_cypher_requests(mut client: Client) {
    let r = client.execute_cypher_request("create (n:Person) return n").await;
    if let Ok(d) = r {
        println!("{}", d.to_string())
    }
    let r = client.execute_cypher_request("create (n:Movie) return n").await;
    if let Ok(d) = r {
        println!("{}", d.to_string())
    }

    
    let r = client.execute_cypher_request("create (n:Person) return n").await;
    if let Ok(d) = r {
        println!("{}", d.to_string())
    }
    let r = client.execute_cypher_request("create (n:Movie) return n").await;
    if let Ok(d) = r {
        println!("{}", d.to_string())
    }
    
    let r = client.execute_cypher_request("match (n:Person) return n").await;
    if let Ok(d) = r {
        let res = d.get_document("result").expect("result");
        let graphs = res.get_array("graphs").expect("graphs");
        assert_eq!(graphs.len(), 2);
        for g in graphs {
            let graph = g.as_document().expect("a graph");
            let nodes = graph.get_array("nodes").expect("nodes");
            assert_eq!(nodes.len(), 1);
        }
    } else {
        assert!(false, "no response")
    }
    
    let r = client.execute_cypher_request("match (n:Movie) return n").await;
    if let Ok(d) = r {
        let res = d.get_document("result").expect("result");
        let graphs = res.get_array("graphs").expect("graphs");
        assert_eq!(graphs.len(), 2);
        for g in graphs {
            let graph = g.as_document().expect("a graph");
            let nodes = graph.get_array("nodes").expect("nodes");
            assert_eq!(nodes.len(), 1);
        }
    } else {
        assert!(false, "no response")
    }
    
    let r = client.execute_cypher_request("match (p:Person), (m:Movie) create (m)<-[r:Played]-(p) return m, r, p").await;
    if let Ok(d) = r {
        println!("{}", d.to_string());
        let res = d.get_document("result").expect("result");
        let graphs = res.get_array("graphs").expect("graphs");
        assert_eq!(graphs.len(), 4);
        for g in graphs {
            let graph = g.as_document().expect("a graph");
            let nodes = graph.get_array("nodes").expect("nodes");
            let relationships = graph.get_array("relationships").expect("relationships");
            assert_eq!(nodes.len(), 2);
            assert_eq!(relationships.len(), 1);
        }
    }
}

async fn test_cypher_requests_complete_graph(mut client: Client) {
    for _ in 0..10 {
        let r = client.execute_cypher_request("create (n:Person) return n").await;
        if let Ok(d) = r {
            println!("{}", d.to_string())
        }
    }

    let r = client.execute_cypher_request("match (x:Person), (y:Person) create (x)-[f:FRIEND_OF]->(y) return f").await;
    if let Ok(d) = r {
        println!("{}", d.to_string());
        let res = d.get_document("result").expect("result");
        let graphs = res.get_array("graphs").expect("graphs");
        assert_eq!(graphs.len(), 100);
        for g in graphs {
            let graph = g.as_document().expect("a graph");
            let relationships = graph.get_array("relationships").expect("relationships");
            assert_eq!(relationships.len(), 1);
        }
    }
}

async fn test_cypher_self_relationship(mut client: Client) {
    let r = client.execute_cypher_request("create (n:Person) return n").await;
    if let Ok(d) = r {
        println!("{}", d.to_string())
    }

    let r = client.execute_cypher_request("match (x:Person) create (x)-[f:FRIEND_OF]->(x) return f, x").await;
    if let Ok(d) = r {
        println!("{}", d.to_string());
        let res = d.get_document("result").expect("result");
        let graphs = res.get_array("graphs").expect("graphs");
        assert_eq!(graphs.len(), 1);
        for g in graphs {
            let graph = g.as_document().expect("a graph");
            let nodes = graph.get_array("nodes").expect("nodes");
            let relationships = graph.get_array("relationships").expect("relationships");
            assert_eq!(nodes.len(), 1);
            assert_eq!(relationships.len(), 1);
        }
    }
}

async fn test_cypher_self_relationship_2(mut client: Client) {
    let r = client.execute_cypher_request("create (n:Person) return n").await;
    if let Ok(d) = r {
        println!("{}", d.to_string())
    }

    let r = client.execute_cypher_request("match (x:Person) create (x)-[f:FRIEND_OF]->(x) return f, x").await;
    if let Ok(d) = r {
        println!("{}", d.to_string());
        let res = d.get_document("result").expect("result");
        let graphs = res.get_array("graphs").expect("graphs");
        assert_eq!(graphs.len(), 1);
        for g in graphs {
            let graph = g.as_document().expect("a graph");
            let nodes = graph.get_array("nodes").expect("nodes");
            let relationships = graph.get_array("relationships").expect("relationships");
            assert_eq!(nodes.len(), 1);
            assert_eq!(relationships.len(), 1);
        }
    }
}

async fn test_double_create_issue(mut client: Client) {
    let r3 = client.execute_cypher_request("create (n:Movie)<-[r:Played]-(p:Person) return n, r, p").await;
    if let Ok(d) = r3 {
        println!("{}", d.to_string());
        let res = d.get_document("result").expect("result");
        let graphs = res.get_array("graphs").expect("graphs");
        for g in graphs {
            let graph = g.as_document().expect("a graph");
            let nodes = graph.get_array("nodes").expect("nodes");
            assert_eq!(nodes.len(), 2);
        }
    } else {
        assert!(false, "no response")
    }
}

async fn test_create_path(mut client: Client) {
     let r = client.execute_cypher_request("create (n:Movie)<-[r:Played]-(p:Person) return n, r, p").await;
    if let Ok(d) = r {
        println!("{}", d.to_string());
        let res = d.get_document("result").expect("result");
        let graphs = res.get_array("graphs").expect("graphs");
        assert_eq!(graphs.len(), 1);
        for g in graphs {
            let graph = g.as_document().expect("a graph");
            let nodes = graph.get_array("nodes").expect("nodes");
            let relationships = graph.get_array("relationships").expect("relationships");
            assert_eq!(nodes.len(), 2);
            assert_eq!(relationships.len(), 1);
        }
    } else {
        assert!(false, "no response")
    }
}

async fn test_mutliple_match(mut client: Client) {
    let r = client.execute_cypher_request("create (n:Movie)<-[r:Played]-(p:Person) return n, r, p").await;
    if let Ok(d) = r {
        println!("{}", d.to_string());
    } else {
        assert!(false, "no response")
    }
    let r = client.execute_cypher_request("match (m:Movie) create (m)<-[r:Produced]-(p:Producer) return m, r, p").await;
    if let Ok(d) = r {
        println!("{}", d.to_string());
    } else {
        assert!(false, "no response")
    }
    let r = client.execute_cypher_request("match (m:Movie)<-[r:Played]-(p:Person) match (m)<-[produced:Produced]-(prd:Producer) return m, r, produced, p, prd").await;
    if let Ok(d) = r {
        println!("{}", d.to_string());
        let res = d.get_document("result").expect("result");
        let graphs = res.get_array("graphs").expect("graphs");
        assert_eq!(graphs.len(), 1);
        for g in graphs {
            let graph = g.as_document().expect("a graph");
            let nodes = graph.get_array("nodes").expect("nodes");
            let relationships = graph.get_array("relationships").expect("relationships");
            assert_eq!(nodes.len(), 3);
            assert_eq!(relationships.len(), 2);
        }
    } else {
        assert!(false, "no response")
    }
}


fn extract_node_id(d: Document) -> Option<i64> {
    let res = d.get_document("result").ok()?;
    let graphs = res.get_array("graphs").ok()?;
    assert_eq!(graphs.len(), 1);
    let mut res = None;
    for g in graphs {
        let graph = g.as_document()?;
        let nodes = graph.get_array("nodes").ok()?;
        for n in nodes {
            let nid = n.as_document()?.get_i64("id").ok();
            res = nid;
        }
        assert_eq!(nodes.len(), 1);
    }
    res
}


async fn test_where_clause_on_ids(mut client: Client) {
    let mut params = Parameters::new();
    let r = client.execute_cypher_request("create (n:Person) return n").await;
    if let Ok(d) = r {
        println!("{}", d.to_string());
        params.insert("pid".to_string(), Value::Integer(extract_node_id(d).expect("pid")));
    }
    let r = client.execute_cypher_request("create (n:Movie) return n").await;
    if let Ok(d) = r {
        println!("{}", d.to_string());
        params.insert("nid".to_string(), Value::Integer(extract_node_id(d).expect("nid")));
    }
    
    
    let r = client.execute_cypher_request_with_parameters("match (n:Movie), (p:Person) where id(n) = $nid and id(p) = $pid create (n:Movie)<-[r:Played]-(p:Person) return n, r, p", params).await;
    if let Ok(d) = r {
        println!("reponse {}", d.to_string());
        let res = d.get_document("result").expect("result");
        let graphs = res.get_array("graphs").expect("graphs");
        assert_eq!(graphs.len(), 1);
        for g in graphs {
            let graph = g.as_document().expect("a graph");
            let nodes = graph.get_array("nodes").expect("nodes");
            let relationships = graph.get_array("relationships").expect("relationships");
            assert_eq!(nodes.len(), 2);
            assert_eq!(relationships.len(), 1);
        }
    } else {
        assert!(false, "no response")
    }

}

# zawgl-client
Zawgl graph database rust client

## Usage
Zawgl query language is Cypher.

Sample usage:
```
let client = Client::new("ws://localhost:8182").await;
let mut params = Parameters::new();
params.insert("pid".to_string(), PropertyValue::Integer(extract_node_id(d).expect("pid")));
let r = client.execute_cypher_request_with_parameters("create (n:Person) where id(p) = $pid return n", params).await;
```
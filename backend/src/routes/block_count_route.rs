use reqwest::{Client};
use axum::{Router, routing::get};
use axum::extract::{State};
use crate::model::block_count::RPCResponse;

const BITCOIN: &str = "http://127.0.0.1:8332";

// this creates a router and client server to retrieve information from the client 
// routes to handler 
pub fn route() -> Router {
    let app = Router::new(); // creates a router
    let state = Client::new(); // used to make HTTP requests 
    app.route("model/block_count", get(block_count_route)) // route needs 2 handler functions
    .with_state(state) // shared HTTP client the router holds onto 
}

// helper function; returns the RPC response
// extractor function 
pub async fn send_request (
    client: &reqwest::Client,
    body: serde_json::Value
) -> Result<block_count::RPCResponse, reqwest::Error>
{
client
    .post(BITCOIN)
    .json(&body) // fetches the json body of the serialized data 
    .send() // sends the request 
    .await? 
    .json::<block_count::RPCResponse>() // deserializes the response
    .await // drop the error check
}

// this retrieves the information from the client
// handler function, aka receiving a request/response
pub async fn block_count(
    State(client): State<Client> // destructures the State wrapper and extracts client
) -> Result<block_count::RPCResponse, reqwest::Error>
{
    // this the reponse from the API request 
    let body = serde_json::json!({ // lets you write JSON directly into Rust code automatically 
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getblockcount",
        "params": [],
    });
    send_request(&client, body).await
}
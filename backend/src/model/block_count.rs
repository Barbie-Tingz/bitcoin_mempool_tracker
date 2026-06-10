use serde:: {Deserialize, Serialize}; 

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all= "camelCase")]
pub struct RPCResponse {
    pub jsonrpc: String,
    pub result: Option<u64>, 
    pub id: u64, 
    pub error: Option<Error>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all= "camelCase")]
pub struct Error {
    pub code: i32, 
    pub message: String,
}
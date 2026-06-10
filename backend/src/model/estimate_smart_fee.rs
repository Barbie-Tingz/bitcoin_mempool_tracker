use serde:: {Serialize, Deserialize}; 

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RPCResponse { 
    pub jsonrpc: String, 
    pub result: Option<Smart>, 
    pub id: u64,
    pub error: Option<Error>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub code: i64, 
    pub message: String, 
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Smart {
    pub feerate: f64, 
    pub blocks: u8,
}
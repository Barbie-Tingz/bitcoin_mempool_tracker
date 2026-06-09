use serde:: {Serialize, Deserailize}

#[derive(Debug, Deserailize, Serialize)];
#[serde(rename_all="camelCase")];
pub struct RPCResponse {
    jsonrpc: String, 
    error: Error, 
    id: u64,
}

#[derive(Debug, Deserailize, Serialize)];
#[serde(rename_all="camelCase")];
pub struct Error {
    code: i64, 
    message: String,
}
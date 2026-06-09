use serde:: {Deserialize, Serialize}; 

#[derive(Debug, Deserialize, Serialize)]
#[derive(rename_all= "camelCase")]
pub struct RPCResponse {
    pub jsonrpc: String,
    pub result: u64, 
    pub: u64, 
}
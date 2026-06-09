use serde:: {Serialize, Deserailize}; 

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RPCResponse { 
    pub jsonrpc: String, 
    pub result: Smart, 
    pub id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Smart {
    pub feerate: f64, 
    pub blocks: u8,
}
use serde:: {Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RPCResponse {
    pub jsonrpc: String, 
    pub result: HashMap<String, Key>, 
    pub id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    pub vsize: u64, 
    pub weight: u64, 
    pub time: u64, 
    pub descendantcount: u64,
    pub descendantsize: u64, 
    pub ancestorcount: u64,
    pub ancestorsize: u64,
    pub wtxid: String,
    pub chunkweight: u64,
    pub fees: Fee,
    pub depends: Vec<String>,
    pub spentby: Vec<String>,
    pub bip125_replaceable: bool,
    pub unbroadcast: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Fee {
    pub base: f64, 
    pub modified: f64, 
    pub ancestor: f64, 
    pub descendant: f64, 
    pub chunk: f64,
}

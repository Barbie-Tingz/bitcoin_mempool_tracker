use serde:: {Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct RPCResponse {
    pub jsonrpc: String, 
    pub result: Option<Transaction>, 
    pub id: u64, 
    pub error: Option<Error>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Error { 
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Transaction {
    pub txid: String, 
    pub hash: String, 
    pub version: u64, 
    pub size: u64, 
    pub vsize: u64, 
    pub weight: u64,
    pub locktime: u64, 
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub hex: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Vin { 
    pub txid: String, 
    pub vout: u64, 
    pub script_sig: Script,
    pub txinwitness: Option<Vec<String>>, 
    pub sequence: u64, 
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Vout {
    pub value: f64, 
    pub n: u64, 
    pub script_pub_key: PubKey, 
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Script {
    pub asm: String,
    pub hex: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct PubKey {
    pub asm: String, 
    pub desc: String, 
    pub hex: String, 
    pub address: Option<String>,
    #[serde(rename = "type")]
    pub script_type: String, 
}
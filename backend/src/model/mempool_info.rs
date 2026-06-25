use serde:: {Deserialize, Serialize}; 

// returns general stats about the mempool (size, fees, etc.)

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all= "camelCase")]
pub struct RPCResponse { 
    pub jsonrpc: String, 
    pub result: Option<Information>, 
    pub id: u64, 
    pub error: Option<Error>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all= "camelCase")]
pub struct Error {
    pub code: i32, 
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all= "camelCase")]
pub struct Information {
    pub loaded: bool, 
    pub size: u64, 
    pub bytes: u64, 
    pub usage: u64, 
    #[serde(rename = "total_fee")]
    pub totalfee: f64, 
    pub maxmempool: u64, 
    pub mempoolminfee: f64,
    pub minrelaytxfee: f64, 
    pub incrementalrelayfee: f64, 
    pub unbroadcastcount: u8, 
    pub fullrbf: bool, 
    pub permitbaremultisig: bool, 
    pub maxdatacarriersize: u64, 
    pub limitclustercount: u64, 
    pub limitclustersize: u64, 
    pub optimal: bool,
}
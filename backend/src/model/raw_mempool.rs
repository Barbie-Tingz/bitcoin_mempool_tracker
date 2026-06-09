use serde:: {Serialize, Deserialize}

#[derive(Debug, Deserialize, Serialize)];
#[serde(rename_all = "camelCase")];
pub struct RPCResponse {

}

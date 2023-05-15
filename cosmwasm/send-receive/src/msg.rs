use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    SendMessageEvm {
        destination_chain: String,
        destination_address: String,
        message: String,
        send_fee: bool,
    },
    SendMessageOsmosis {
        destination_chain: String,
        destination_address: String,
        message: String,
        send_fee: bool,
    },
    ReceiveMessageOsmosis {},
    ReceiveMessageEvm {
        source_chain: String,
        source_address: String,
        payload: Binary
    },

}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_serde]
pub struct Fee {
    pub amount: String,
    pub recipient: String,
}

#[cw_serde]
pub struct GmpMessage {
    pub destination_chain: String,
    pub destination_address: String,
    pub payload: Vec<u8>,
    #[serde(rename = "type")]
    pub type_: i64,
    pub fee: Option<Fee>,
}

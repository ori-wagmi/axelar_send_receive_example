#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use ethabi::{decode, encode, ParamType, Token};
use serde_json_wasm::to_string;

// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::*;
use crate::state::*;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:send-receive";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        SendMessageEvm {
            destination_chain,
            destination_address,
            message,
        } => exec::send_message_evm(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            message,
        ),
        SendMessageOsmosis {
            destination_chain,
            destination_address,
            message,
            send_fee,
        } => exec::send_message_osmosis(
            deps,
            env,
            info,
            destination_chain,
            destination_address,
            message,
            send_fee,
        ),
        ReceiveMessageEvm {
            source_chain,
            source_address,
            payload,
        } => exec::receive_message_evm(deps, source_chain, source_address, payload),
        ReceiveMessageOsmosis {} => exec::receive_message_osmosis(),
    }
}

mod exec {
    use super::*;

    // Sends a message via Axelar GMP to the EVM {destination_chain} and {destination_address}
    pub fn send_message_evm(
        _deps: DepsMut,
        env: Env,
        info: MessageInfo,
        destination_chain: String,
        destination_address: String,
        message: String,
    ) -> Result<Response, ContractError> {
        // Message payload to be received by the destination
        let message_payload = encode(&vec![
            Token::String(info.sender.to_string()),
            Token::String(message),
        ]);

        // {info.funds} used to pay gas. Must only contain 1 token type.
        let coin: cosmwasm_std::Coin = cw_utils::one_coin(&info).unwrap();

        let gmp_message: GmpMessage = GmpMessage {
            destination_chain,
            destination_address,
            payload: message_payload.to_vec(),
            type_: 1,
            fee: None,
        };

        let ibc_message = crate::ibc::MsgTransfer {
            source_port: "transfer".to_string(),
            source_channel: "channel-3".to_string(), // Testnet Osmosis to axelarnet: https://docs.axelar.dev/resources/testnet#ibc-channels
            token: Some(coin.into()),
            sender: env.contract.address.to_string(),
            receiver: "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5"
                .to_string(),
            timeout_height: None,
            timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
            memo: to_string(&gmp_message).unwrap(),
        };

        Ok(Response::new().add_message(ibc_message))
    }

    // Sends a message via Axelar GMP to the Osmosis {destination_chain} and {destination_address}
    // only difference is how the {message_payload} is constructed
    pub fn send_message_osmosis(
        _deps: DepsMut,
        env: Env,
        info: MessageInfo,
        destination_chain: String,
        destination_address: String,
        _message: String,
        send_fee: bool,
    ) -> Result<Response, ContractError> {
        // Message payload for Osmosis -> Osmosis GMP
        let contract_call = serde_json_wasm::to_string(&ExecuteMsg::ReceiveMessageOsmosis {})
            .expect("Failed to serialize struct to JSON");
        let utf8_bytes = contract_call.as_bytes();
        let utf8_vec = utf8_bytes.to_owned();
        let mut message_payload: Vec<u8> = vec![0, 0, 0, 2];
        message_payload.extend(utf8_vec);

        // info.funds used to pay gas. Must only contain 1 token type.
        let coin: cosmwasm_std::Coin = cw_utils::one_coin(&info).unwrap();

        // if send_fee -> include GmpMessage.fee field, otherwise None
        if send_fee {
            // Generate gmp message to be included in the IBC memo

            let gmp_message: GmpMessage = GmpMessage {
                destination_chain,
                destination_address,
                payload: message_payload.to_vec(),
                type_: 1,
                fee: Some(Fee {
                    amount: coin.amount.to_string(),
                    recipient: "axelar1uu8enfg5tq7ycnwyk7zcd72aq7fk0fcx4p2sdm".to_string(), // Axelar gas receiver
                }),
            };

            // Generate IBC message to be sent to the Axelar Gateway
            let ibc_message = crate::ibc::MsgTransfer {
                source_port: "transfer".to_string(),
                source_channel: "channel-3".to_string(), // Testnet Osmosis to axelarnet: https://docs.axelar.dev/resources/testnet#ibc-channels
                token: Some(coin.into()),
                sender: env.contract.address.to_string(),
                receiver: "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5"
                    .to_string(), // Axelar Gateway
                timeout_height: None,
                timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
                memo: to_string(&gmp_message).unwrap(),
            };

            Ok(Response::new().add_message(ibc_message))
        } else {
            let gmp_message: GmpMessage = GmpMessage {
                destination_chain,
                destination_address,
                payload: message_payload.to_vec(),
                type_: 1,
                fee: None,
            };

            let ibc_message = crate::ibc::MsgTransfer {
                source_port: "transfer".to_string(),
                source_channel: "channel-3".to_string(), // Testnet Osmosis to axelarnet: https://docs.axelar.dev/resources/testnet#ibc-channels
                token: Some(coin.into()),
                sender: env.contract.address.to_string(),
                receiver: "axelar1dv4u5k73pzqrxlzujxg3qp8kvc3pje7jtdvu72npnt5zhq05ejcsn5qme5"
                    .to_string(),
                timeout_height: None,
                timeout_timestamp: Some(env.block.time.plus_seconds(604_800u64).nanos()),
                memo: to_string(&gmp_message).unwrap(),
            };

            Ok(Response::new().add_message(ibc_message))
        }
    }

    pub fn receive_message_evm(
        deps: DepsMut,
        _source_chain: String,
        _source_address: String,
        payload: Binary,
    ) -> Result<Response, ContractError> {
        // decode the payload
        // executeMsgPayload: [sender, message]
        let decoded = decode(
            &vec![ParamType::String, ParamType::String],
            payload.as_slice(),
        )
        .unwrap();

        // store message
        STORED_MESSAGE.save(
            deps.storage,
            &Message {
                sender: decoded[0].to_string(),
                message: decoded[1].to_string(),
            },
        )?;

        Ok(Response::new())
    }

    pub fn receive_message_osmosis() -> Result<Response, ContractError> {
        Ok(Response::new())
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        GetStoredMessage {} => to_binary(&query::get_stored_message(deps)?),
    }
}

mod query {
    use super::*;

    pub fn get_stored_message(deps: Deps) -> StdResult<GetStoredMessageResp> {
        let message = STORED_MESSAGE.may_load(deps.storage).unwrap().unwrap();
        let resp = GetStoredMessageResp {
            sender: message.sender,
            message: message.message,
        };
        Ok(resp)
    }
}

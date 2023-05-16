# Cosmwasm SendReceive Smart Contract
This project contains the Cosmwasm smart contract that can send and receive message payloads to/from EVM.

This contract is deployed to osmosis-5 testnet: `osmo1ejnsp7uk2yrrswrxktav7gdayqc2qmtjh42xlmmzd9965f7exzpqdnm0h8`

This contract takes inspiration from: https://github.com/axelarnetwork/evm-cosmos-gmp-sample

## How to use
### Osmosisd CLI
You can interact with the contract using osmosisd CLI: https://docs.osmosis.zone/osmosis-core/osmosisd/

### Tests
Unit tests can be run with `cargo test`

## Osmosis -> Osmosis GMP
This contract also contains the logic for Osmosis -> Osmosis GMP in `send_message_osmosis`. However, gas payment for this scenario is still unsolved so it currently isn't supported.


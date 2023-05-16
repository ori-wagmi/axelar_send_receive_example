# EVM SendReceive smart contract

This project contains the EVM smart contract that can send and receive message payloads to/from Cosmwasm.

This contract is deployed to Fuji Testnet: https://testnet.snowtrace.io/address/0x944bd5Df4bC447f37F52d4CaD89B805DC082aB40#code

This contract takes inspiration from: https://github.com/axelarnetwork/evm-cosmos-gmp-sample

## How to use
User must first create a `.env` file that contains 
```
DEPLOYER_PRIV_KEY = <deployer_wallet_private_key>
FUJI_API_KEY = <infura_api_key_for_fuji_testnet>
```

### Deploy
SendReceive can be deployed to Fuji Testnet using `npx hardhat run ./scripts/deploy.js --network fuji`.

### Tests
Local tests use a mock gateway/gas service found at `/contracts/Mock/AxelarGatewayGasServiceMock.sol`. This is used to mock out all calls from the SendReceive contract to the Gateway and GasService in testing.

Tests can be run using `npx hardhat test`



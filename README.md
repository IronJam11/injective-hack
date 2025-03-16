# Injective Counter Contract Example

This repo showcases an example of how to implement the connection and interact with a Smart Contract deployed on the Injective Chain using the `injective-ts` module using Nuxt and Next.

More about `injective-ts` here: [injective-ts wiki](https://github.com/InjectiveLabs/injective-ts/wiki)

Link to the Smart Contract Repo: [cw-counter](https://github.com/InjectiveLabs/cw-counter)

In This README is an example of how to implement the connection and interact with the smart contract in VanillaJS.
You can find README's for Nuxt And Next.js examples in the folders respectively.

## 1. Preparation

Start by installing the node module dependencies you are going to use (like `@injectivelabs/sdk-ts` etc...)

We can see the modules used in this example in `package.json`

Before we start using the `@injectivelabs` modules, we first need to configure our bundler, by adding some plugins like `node-modules-polyfill`.

## 2. Setting up the Services

Next, we need to setup the services we are going to use.
For interacting with the smart contract, we are going to use `ChainGrpcWasmApi` from `@injectivelabs/sdk-ts`.
Also we will need the Network Endpoints we are going to use (Mainnet or Testnet), which we can find in `@injectivelabs/networks`

Example:

```js
import { ChainGrpcWasmApi } from "@injectivelabs/sdk-ts";
import { Network, getNetworkEndpoints } from "@injectivelabs/networks";

export const NETWORK = Network.TestnetK8s;
export const ENDPOINTS = getNetworkEndpoints(NETWORK);

export const chainGrpcWasmApi = new ChainGrpcWasmApi(ENDPOINTS.grpc);
```

## 3. Wallet Strategy and Broadcast

Next we need to setup Wallet Strategy And Broadcasting Client, by importing `WalletStrategy` and `MsgBroadcaster` from `@injectivelabs/wallet-ts`.

The main purpose of the `@injectivelabs/wallet-ts` is to offer developers a way to have different wallet implementations on Injective. All of these wallets implementations are exposing the same `ConcreteStrategy` interface which means that users can just use these methods without the need to know the underlying implementation for specific wallets as they are abstracted away.

To start, you have to make an instance of the WalletStrategy class which gives you the ability to use different wallets out of the box. You can switch the current wallet that is used by using the setWallet method on the walletStrategy instance.
The default is `Metamask`.

```js
import { WalletStrategy } from "@injectivelabs/wallet-ts";
import { Web3Exception } from "@injectivelabs/exceptions";

// These imports are from .env
import {
  CHAIN_ID,
  ETHEREUM_CHAIN_ID,
  IS_TESTNET,
  alchemyRpcEndpoint,
  alchemyWsRpcEndpoint,
} from "/constants";

export const walletStrategy = new WalletStrategy({
  chainId: CHAIN_ID,
  ethereumOptions: {
    ethereumChainId: ETHEREUM_CHAIN_ID,
    wsRpcUrl: alchemyWsRpcEndpoint,
    rpcUrl: alchemyRpcEndpoint,
  },
});
```

To get the addresses from the wallet we can use the following function:

```js
export const getAddresses = async (): Promise<string[]> => {
  const addresses = await walletStrategy.getAddresses();

  if (addresses.length === 0) {
    throw new Web3Exception(
      new Error("There are no addresses linked in this wallet.")
    );
  }

  return addresses;
};
```

When we call this function it opens up your Wallet (Default: Metamask) so we can connect, and we can store the return value in a variable for later use.
Note that this returns and array of Ethereum Addresses, which we can convert to injective addresses using the `getInjectiveAddress` utility function.

```js
import {getInjectiveAddress} from '@injectivelabs/sdk-ts'

const [address] = await getAddresses();
cosnt injectiveAddress = getInjectiveAddress(getInjectiveAddress)

```

We will also need the `walletStrategy` for the `BroadcastClient`, including the networks used, which we defined earlier.

```js
import { Network } from "@injectivelabs/networks";
export const NETWORK = Network.TestnetK8s;

export const msgBroadcastClient = new MsgBroadcaster({
  walletStrategy,
  network: NETWORK,
});
```

## 4. Querying the Smart Contract

Now everything is setup and we can interact with the Smart Contract.

We will begin by Quering the The Smart Contract to get the current count using the `chainGrpcWasmApi` service we created earlier,
and calling `get_count` on the Smart Contract.

```js

      const response = (await chainGrpcWasmApi.fetchSmartContractState(
        COUNTER_CONTRACT_ADDRESS, // The address of the contract
        toBase64({ get_count: {} }) // We need to convert our query to Base64
      )) as { data: string };

      const { count } = fromBase64(response.data) as { count: number }; // we need to convert the response from Base64

      console.log(count)
```

## 5. Modifying the State

Next we will modify the `count` state.
We can do that by sending messages to the chain using the `Broadcast Client` we created earlier and `MsgExecuteContractCompat` from `@injectivelabs/sdk-ts`

The Smart Contract we use for this example has 2 methods for altering the state:

- `increment`
- `reset`

`increment` increment the count by 1, and `reset` sets the count to a given value.
Note that `reset` can only be called if you are the creator of the smart contract.

When we call these functions, our wallet opens up to sign the message/transaction and broadcasts it.

```js
// Preparing the message

const msg = MsgExecuteContractCompat.fromJSON({
  contractAddress: COUNTER_CONTRACT_ADDRESS,
  sender: injectiveAddress,
  msg: {
    increment: {},
  },
});

// Signing and broadcasting the message

await msgBroadcastClient.broadcast({
  msgs: msg,
  injectiveAddress: injectiveAddress,
});
```

```js
// Preparing the message

const msg = MsgExecuteContractCompat.fromJSON({
  contractAddress: COUNTER_CONTRACT_ADDRESS,
  sender: injectiveAddress,
  msg: {
    reset: {
      count: parseInt(number, 10),
    },
  },
});

// Signing and broadcasting the message

await msgBroadcastClient.broadcast({
  msgs: msg,
  injectiveAddress: injectiveAddress,
});
```

## 6.Full Example

Below is an example of everything we learned so far, put together.

```js
import { ChainGrpcWasmApi, getInjectiveAddress } from "@injectivelabs/sdk-ts";
import { Network, getNetworkEndpoints } from "@injectivelabs/networks";
import { WalletStrategy } from "@injectivelabs/wallet-ts";
import { Web3Exception } from "@injectivelabs/exceptions";

// These imports are from .env
import {
  CHAIN_ID,
  ETHEREUM_CHAIN_ID,
  IS_TESTNET,
  alchemyRpcEndpoint,
  alchemyWsRpcEndpoint,
} from "/constants";

const NETWORK = Network.TestnetK8s;
const ENDPOINTS = getNetworkEndpoints(NETWORK);

const chainGrpcWasmApi = new ChainGrpcWasmApi(ENDPOINTS.grpc);

const walletStrategy = new WalletStrategy({
  chainId: CHAIN_ID,
  ethereumOptions: {
    ethereumChainId: ETHEREUM_CHAIN_ID,
    wsRpcUrl: alchemyWsRpcEndpoint,
    rpcUrl: alchemyRpcEndpoint,
  },
});

export const getAddresses = async (): Promise<string[]> => {
  const addresses = await walletStrategy.getAddresses();

  if (addresses.length === 0) {
    throw new Web3Exception(
      new Error("There are no addresses linked in this wallet.")
    );
  }

  return addresses;
};

const msgBroadcastClient = new MsgBroadcaster({
  walletStrategy,
  network: NETWORK,
});

const [address] = await getAddresses();
const injectiveAddress = getInjectiveAddress(getInjectiveAddress);

async function fetchCount() {
  const response = (await chainGrpcWasmApi.fetchSmartContractState(
    COUNTER_CONTRACT_ADDRESS, // The address of the contract
      toBase64({ get_count: {} }) // We need to convert our query to Base64
    )) as { data: string };

  const { count } = fromBase64(response.data) as { count: number }; // we need to convert the response from Base64

  console.log(count)
}

async function increment(){
    const msg = MsgExecuteContractCompat.fromJSON({
    contractAddress: COUNTER_CONTRACT_ADDRESS,
    sender: injectiveAddress,
    msg: {
        increment: {},
        },
    });

    // Signing and broadcasting the message

    await msgBroadcastClient.broadcast({
        msgs: msg,
        injectiveAddress: injectiveAddress,
    });
}

async function main() {
    await fetchCount() // this will log: {count: 5}
    await increment() // this opens up your wallet to sign the transaction and broadcast it
    await fetchCount() // the count now is 6. log: {count: 6}
}

main()

```

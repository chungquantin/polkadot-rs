# `polkadot-rs`
Rust implementation of the `@polkadot-js/api`
## @polkadot/rpc-provider

Generic transport providers to handle the transport of method calls to and from Polkadot clients from applications interacting with it. It provides an interface to making RPC calls and is generally, unless you are operating at a low-level and taking care of encoding and decoding of parameters/results, it won't be directly used, rather only passed to a higher-level interface.

### Provider Selection

There are three flavours of the providers provided, one allowing for using HTTP as a transport mechanism, the other using WebSockets, and the third one uses substrate light-client through @substrate/connect. It is generally recommended to use the [[WsProvider]] since in addition to standard calls, it allows for subscriptions where all changes to state can be pushed from the node to the client.

All providers are usable (as is the API), in both browser-based and Node.js environments. Polyfills for unsupported functionality are automatically applied based on feature-detection.

### Usage

Installation -

```
yarn add @polkadot/rpc-provider
```

WebSocket Initialization -

```rust
use rpc_provider::{defaults::WS_URL, jsonrpsee::JsonrpseeClient, rpc_params, Request};
use sp_core::H256;
use types_support::metadata::v15::polkadot_rpc::PolkadotRpcMethod;

#[tokio::main]
async fn main() {
	let client = JsonrpseeClient::new(WS_URL).await.unwrap();
	let method = PolkadotRpcMethod::ChainGetBlockHash.as_string();
	let output = client.request::<H256>(&method, rpc_params!(Some(0))).await.unwrap();

	println!("Blockhas: {output:?}");
}
```

HTTP Initialization -

```rust
use rpc_provider::{
	defaults::HTTP_URL, http::HttpProvider, rpc_params, types::ProviderInterface, Request,
};
use sp_core::H256;
use types_support::metadata::v15::polkadot_rpc::PolkadotRpcMethod;

#[tokio::main]
async fn main() {
	let mut provider = HttpProvider::new(HTTP_URL).unwrap();

	provider.connect().await.unwrap();

	let method = PolkadotRpcMethod::ChainGetBlockHash.as_string();
	let output = provider.request::<H256>(&method, rpc_params!(Some(0))).await.unwrap();

	println!("Blockhash: {output:?}");
}
```

`smoldot` Light Client Initialization -

Instantiating a Provider for the Polkadot Relay Chain:

```javascript
use rpc_provider::{light_client::ScProvider, rpc_params, Request};
use sp_core::H256;
use types_support::metadata::v15::polkadot_rpc::PolkadotRpcMethod;

#[tokio::main]
async fn main() {
	let chain_spec = include_str!("../../../chain_spec/demo/polkadot.json");

	let provider = ScProvider::new(chain_spec, vec![]);
	let method = PolkadotRpcMethod::ChainGetBlockHash.as_string();
	let output = provider.request::<H256>(&method, rpc_params!(Some(0))).await.unwrap();

	println!("Blockhash: {output:?}");
}
```

Instantiating a Provider for a Polkadot parachain:

```javascript
use rpc_provider::{light_client::ScProvider, rpc_params, Request};
use types_support::metadata::v15::polkadot_rpc::PolkadotRpcMethod;

#[tokio::main]
async fn main() {
	let method = PolkadotRpcMethod::ChainGetBlockHash.as_string();

	let relaychain_spec = include_str!("../../../chain_spec/demo/westend.json");
	let relaychain_provider = ScProvider::new(relaychain_spec, vec![]);

	let output = relaychain_provider.request_raw(&method, rpc_params!(Some(0))).await.unwrap();
	println!("Polkadot Blockhash: {output:?}");

	let parachain_chainspec = include_str!("../../../chain_spec/demo/westend-westmint.json");
	let parachain_provider = ScProvider::new(parachain_chainspec, vec![relaychain_provider.id()]);

	let output = parachain_provider.request_raw(&method, rpc_params!(Some(0))).await.unwrap();
	println!("Parachain Blockhash: {output:?}");
}
```


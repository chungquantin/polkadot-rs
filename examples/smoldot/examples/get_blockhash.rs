use rpc_provider::{rpc_params, smoldot::SmoldotLightClient, Request};
use sp_core::H256;
use types_support::metadata::v15::polkadot_rpc::PolkadotRpcMethod;

#[tokio::main]
async fn main() {
	let mut client = SmoldotLightClient::with_default_platform();
	let chain_spec = include_str!("../../../chain_spec/demo/polkadot.json");
	let connection = client.connect_relaychain(chain_spec).unwrap();
	let method = PolkadotRpcMethod::ChainGetBlockHash.as_string();
	let output = client
		.use_connection(connection)
		.request::<H256>(&method, rpc_params!(Some(0)))
		.await
		.unwrap();

	println!("Blockhash: {output:?}");
}

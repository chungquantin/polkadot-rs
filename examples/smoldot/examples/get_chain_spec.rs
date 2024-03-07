use rpc_provider::{rpc_params, smoldot::SmoldotLightClient, Request};
use types_support::{
	chain_spec::ChainSpecMetadata, metadata::v15::polkadot_rpc::PolkadotRpcMethod,
};

#[tokio::main]
async fn main() {
	let mut client = SmoldotLightClient::with_default_platform();

	let chain_spec_path = include_str!("../../../chain_spec/demo/polkadot.json");
	let connection = client.connect_relaychain(chain_spec_path).unwrap();

	let method = PolkadotRpcMethod::SyncStateGenSyncSpec.as_string();
	let output = client
		.use_connection(connection)
		.request::<ChainSpecMetadata>(&method, rpc_params!(Some(0)))
		.await
		.unwrap();

	println!("Blockhash: {output:?}");
}

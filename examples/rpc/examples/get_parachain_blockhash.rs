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

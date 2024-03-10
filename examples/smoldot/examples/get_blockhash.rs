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

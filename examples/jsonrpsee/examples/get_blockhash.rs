use rpc_provider::{
	defaults::WS_URL, rpc_params, types::ProviderInterface, ws::WsProvider, Request,
};
use sp_core::H256;
use types_support::metadata::v15::polkadot_rpc::PolkadotRpcMethod;

#[tokio::main]
async fn main() {
	let mut provider = WsProvider::new(WS_URL).unwrap();

	provider.connect().await.unwrap();

	let method = PolkadotRpcMethod::ChainGetBlockHash.as_string();
	let output = provider.request::<H256>(&method, rpc_params!(Some(0))).await.unwrap();

	println!("Blockhash: {output:?}");
}

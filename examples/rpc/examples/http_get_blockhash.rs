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

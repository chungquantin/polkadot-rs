use primitives::RpcParams;
use serde::de::DeserializeOwned;

pub mod defaults;
pub mod error;
pub mod mac;
pub mod primitives;
pub mod rpc;
pub use error::{Error, Result};

pub use rpc::*;

/// Trait to be implemented by the ws-client for sending rpc requests and extrinsic.
#[maybe_async::maybe_async(?Send)]
pub trait Request {
	/// Sends a RPC request to the substrate node and returns the answer as deserializable struct (see serde::de::DeserializeOwned).
	async fn request<R: DeserializeOwned>(&self, method: &str, params: RpcParams) -> Result<R>;
	/// Sends a RPC request to the substrate node and returns the answer as JSON string
	async fn request_raw(&self, method: &str, params: RpcParams) -> Result<String>;
}

/// Trait to be implemented by the ws-client for subscribing to the substrate node.
#[maybe_async::maybe_async(?Send)]
pub trait Subscribe {
	type Subscription<Notification>: HandleSubscription<Notification>
	where
		Notification: DeserializeOwned;

	async fn subscribe<Notification: DeserializeOwned>(
		&self,
		sub: &str,
		params: RpcParams,
		unsub: &str,
	) -> Result<Self::Subscription<Notification>>;
}

#[maybe_async::maybe_async(?Send)]
pub trait HandleSubscription<Notification: DeserializeOwned> {
	/// Returns the next notification from the stream.
	async fn next(&mut self) -> Option<Result<Notification>>;

	/// Unsubscribe and consume the subscription.
	async fn unsubscribe(self) -> Result<()>;
}

pub fn to_json_req(method: &str, params: RpcParams) -> Result<String> {
	Ok(serde_json::json!({
		"method": method,
		"params": params.to_json_value()?,
		"jsonrpc": "2.0",
		"id": "1",
	})
	.to_string())
}

#[cfg(test)]
mod tests {
	use sp_core::H256;

	use crate::{defaults::WS_URL, rpc::jsonrpsee::JsonrpseeClient, rpc_params, Request};

	#[tokio::test]
	async fn it_works() {
		let client = JsonrpseeClient::new(WS_URL).await.unwrap();
		assert!(client.is_connected());

		let block_hash: Option<H256> =
			client.request("chain_getBlockHash", rpc_params![Some(0)]).await.unwrap();

		println!("{:?}", block_hash);
	}
}

use std::sync::Arc;

use rpc_provider::types::ProviderInterface;

struct RpcCoreBuilder<P: ProviderInterface> {
	provider: Option<Arc<P>>,
}

impl<P: ProviderInterface> Default for RpcCoreBuilder<P> {
	fn default() -> Self {
		Self { provider: None }
	}
}

impl<P: ProviderInterface> RpcCoreBuilder<P> {
	pub fn add_provider(&mut self, provider: P) -> &mut Self {
		self.provider = Some(Arc::new(provider));
		return self;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rpc_provider::{defaults::WS_URL, ws::WsProvider};

	#[tokio::test]
	async fn it_works() {
		let mut provider = WsProvider::new(WS_URL).unwrap();

		provider.connect().await.unwrap();

		RpcCoreBuilder::default().add_provider(provider);
	}
}

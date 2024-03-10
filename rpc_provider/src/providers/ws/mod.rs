use crate::{
	defaults::WS_URL,
	types::{ProviderInterface, RpcParamsWrapper},
	Error, Request, Result, RpcParams, Subscribe,
};
use jsonrpsee::{
	client_transport::ws::{Url, WsTransportClientBuilder},
	core::client::{Client, ClientBuilder, ClientT, Error as JsonrpseeError, SubscriptionClientT},
};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use subscription::SubscriptionWrapper;

mod subscription;

#[derive(Clone)]
pub struct WsProvider {
	inner: Option<Arc<Client>>,
	_url: Option<Url>,
}

impl ProviderInterface for WsProvider {
	async fn connect(&mut self) -> Result<()> {
		let (tx, rx) = WsTransportClientBuilder::default()
			.build(self._url.clone().unwrap())
			.await
			.map_err(|e| Error::Client(Box::new(e)))?;
		let client = ClientBuilder::default()
			.max_buffer_capacity_per_subscription(4096)
			.build_with_tokio(tx, rx);

		self.inner = Some(Arc::new(client));
		return Ok(());
	}

	async fn disconnect(&mut self) -> Result<()> {
		unimplemented!()
	}
}

impl WsProvider {
	/// Create a new client to a local Substrate node with default port.
	pub async fn with_default_url() -> Result<Self> {
		let mut new_client = Self::new(WS_URL).unwrap();
		new_client.connect().await.unwrap();
		return Ok(new_client);
	}

	pub fn inner(&self) -> Arc<Client> {
		return self.inner.clone().unwrap();
	}

	/// Create a new client with the given url string.
	/// Example url input: "ws://127.0.0.1:9944"
	pub fn new(url: &str) -> Result<Self> {
		let parsed_url: Url = url.parse().map_err(|e| Error::Client(Box::new(e)))?;
		Ok(Self { inner: None, _url: Some(parsed_url) })
	}

	/// Create a new client with the given address, port and max number of reconnection attempts.
	/// Example input:
	/// - address: "ws://127.0.0.1"
	/// - port: 9944
	pub async fn new_with_port(address: &str, port: u32) -> Result<Self> {
		let url = format!("{address}:{port:?}");
		Self::new(&url)
	}

	/// Create a new client with a user-generated Jsonrpsee Client.
	pub fn new_with_client(client: Client) -> Self {
		let inner = Some(Arc::new(client));
		Self { inner, _url: None }
	}

	/// Checks if the client is connected to the target.
	pub fn is_connected(&self) -> bool {
		self.inner().is_connected()
	}

	/// This is similar to [`Client::on_disconnect`] but it can be used to get
	/// the reason why the client was disconnected but it's not cancel-safe.
	///
	/// The typical use-case is that this method will be called after
	/// [`Client::on_disconnect`] has returned in a "select loop".
	///
	/// # Cancel-safety
	///
	/// This method is not cancel-safe
	pub async fn disconnect_reason(&self) -> JsonrpseeError {
		self.inner().disconnect_reason().await
	}

	/// Completes when the client is disconnected or the client's background task encountered an error.
	/// If the client is already disconnected, the future produced by this method will complete immediately.
	///
	/// # Cancel safety
	///
	/// This method is cancel safe.
	pub async fn on_disconnect(&self) {
		self.inner().on_disconnect().await;
	}
}

#[maybe_async::async_impl(?Send)]
impl Request for WsProvider {
	async fn request_raw(&self, method: &str, params: RpcParams) -> Result<String> {
		self.request::<String>(method, params).await
	}

	async fn request<R: DeserializeOwned>(&self, method: &str, params: RpcParams) -> Result<R> {
		self.inner()
			.request(method, RpcParamsWrapper(params))
			.await
			.map_err(|e| Error::Client(Box::new(e)))
	}
}

#[maybe_async::async_impl(?Send)]
impl Subscribe for WsProvider {
	type Subscription<Notification> = SubscriptionWrapper<Notification> where Notification: DeserializeOwned;

	async fn subscribe<Notification: DeserializeOwned>(
		&self,
		sub: &str,
		params: RpcParams,
		unsub: &str,
	) -> Result<Self::Subscription<Notification>> {
		self.inner()
			.subscribe(sub, RpcParamsWrapper(params), unsub)
			.await
			.map(|sub| sub.into())
			.map_err(|e| Error::Client(Box::new(e)))
	}
}

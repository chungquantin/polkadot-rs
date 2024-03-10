use crate::{
	defaults::WS_URL,
	types::{ProviderInterface, RpcParamsWrapper},
	Error, Request, Result, RpcParams,
};
use jsonrpsee::core::client::ClientT;
use jsonrpsee_http_client::{HttpClient, HttpClientBuilder};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use url::Url;

#[derive(Clone)]
pub struct HttpProvider {
	inner: Option<Arc<HttpClient>>,
	_url: Option<Url>,
}

impl ProviderInterface for HttpProvider {
	async fn connect(&mut self) -> Result<()> {
		let url = self._url.clone().unwrap();
		let client = HttpClientBuilder::default().build(url).unwrap();
		self.inner = Some(Arc::new(client));
		return Ok(());
	}

	async fn disconnect(&mut self) -> Result<()> {
		unimplemented!()
	}
}

impl HttpProvider {
	/// Create a new client to a local Substrate node with default port.
	pub async fn with_default_url() -> Result<Self> {
		let mut new_client = Self::new(WS_URL).unwrap();
		new_client.connect().await.unwrap();
		return Ok(new_client);
	}

	pub fn inner(&self) -> Arc<HttpClient> {
		return self.inner.clone().unwrap();
	}

	pub fn new(url: &str) -> Result<Self> {
		let parsed_url: Url = url.parse().map_err(|e| Error::Client(Box::new(e)))?;
		Ok(Self { inner: None, _url: Some(parsed_url) })
	}

	pub async fn new_with_port(address: &str, port: u32) -> Result<Self> {
		let url = format!("{address}:{port:?}");
		Self::new(&url)
	}

	pub fn new_with_client(client: HttpClient) -> Self {
		let inner = Some(Arc::new(client));
		Self { inner, _url: None }
	}
}

#[maybe_async::async_impl(?Send)]
impl Request for HttpProvider {
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

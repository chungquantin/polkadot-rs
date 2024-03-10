//! Reference: https://github.com/smol-dot/smoldot/blob/main/light-base/examples/basic.rs
use crate::{
	error::RpcError, primitives::RpcParams, to_json_req, types::ProviderInterface, Error, Request,
	Result,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use smoldot_light::{
	platform::{DefaultPlatform, PlatformRef},
	AddChainSuccess, ChainId, JsonRpcResponses,
};
use std::{
	fmt::Debug,
	num::NonZeroU32,
	sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct SuccessChainConnection<P: PlatformRef>(Arc<Mutex<JsonRpcResponses<P>>>);

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse<R>
where
	R: Debug,
{
	id: String,
	jsonrpc: String,
	result: Option<R>,
	error: Option<RpcError>,
}

impl<P, TChain> ProviderInterface for ScProvider<Arc<P>, TChain>
where
	Arc<P>: PlatformRef,
{
	async fn connect(&mut self) -> Result<()> {
		return Ok(());
	}

	async fn disconnect(&mut self) -> Result<()> {
		return Ok(());
	}
}

#[derive(Clone)]
pub struct ScProvider<P, TChain>
where
	P: PlatformRef,
{
	inner: Arc<Mutex<smoldot_light::Client<P, TChain>>>,
	id: Option<ChainId>,
	_connection: Option<SuccessChainConnection<P>>,
	_chain_spec: Option<String>,
}

impl<P> ScProvider<Arc<P>, ()>
where
	Arc<P>: PlatformRef,
{
	/// Default platform is a "plug and play" platfrom use environments of your device
	pub fn new(
		chain_spec: &'static str,
		ids: Vec<ChainId>,
	) -> ScProvider<Arc<DefaultPlatform>, ()> {
		let (client_name, client_version) =
			(env!("CARGO_PKG_NAME").into(), env!("CARGO_PKG_VERSION").into());
		let platfrom = DefaultPlatform::new(client_name, client_version);
		let client = smoldot_light::Client::new(platfrom);

		let mut provider = ScProvider {
			id: None,
			inner: Arc::new(Mutex::new(client)),
			_chain_spec: Some(chain_spec.to_string()),
			_connection: None,
		};

		// Add a chain to the provider
		provider.add_chain(chain_spec.to_string(), (), Some(ids)).unwrap();

		return provider;
	}
}

impl<P, TChain> ScProvider<Arc<P>, TChain>
where
	Arc<P>: PlatformRef,
{
	/// Any advance usage will likely require a custom implementation of these bindings.
	pub fn new_with_platform(
		platform: P,
		chain_spec: String,
		user_data: TChain,
		ids: Vec<ChainId>,
	) -> Self {
		let platform = Arc::new(platform);
		let client = smoldot_light::Client::new(platform);
		let mut provider = ScProvider::<Arc<P>, TChain> {
			id: None,
			inner: Arc::new(Mutex::new(client)),
			_chain_spec: Some(chain_spec.clone()),
			_connection: None,
		};
		// Add a chain to the provider
		provider.add_chain(chain_spec, user_data, Some(ids)).unwrap();

		return provider;
	}

	/// Connect to Substrate-based blockchain
	/// - `relay_chains``: If the chain spec is for the parachain, we need to add the relaychain that parachain relies on
	/// Because the `Client` might contain multiple different chains whose similar identifier
	fn add_chain(
		&mut self,
		chain_spec: String,
		user_data: TChain,
		relay_chains: Option<Vec<ChainId>>,
	) -> Result<()> {
		// Ask the client to connect to Polkadot.
		let mut guarded_client = self.inner.lock().unwrap();
		let add_chain_success = guarded_client
			.add_chain(smoldot_light::AddChainConfig {
				specification: &chain_spec,
				// JSON RPC is always enabled in the context of RPC provider
				json_rpc: smoldot_light::AddChainConfigJsonRpc::Enabled {
					// Maximum number of JSON-RPC in the queue of requests waiting to be processed.
					max_pending_requests: NonZeroU32::new(u32::max_value()).unwrap(),
					// Maximum number of active subscriptions before new ones are automatically rejected.
					max_subscriptions: u32::max_value(),
				},
				potential_relay_chains: relay_chains.unwrap_or_default().into_iter(),
				database_content: "",
				user_data,
			})
			.unwrap();

		let AddChainSuccess { chain_id, json_rpc_responses } = add_chain_success;
		// If connected chain does not respond, throw error
		let Some(rpc_responses) = json_rpc_responses else {
			return Err(Error::InvalidChainSpec(format!(
				"Could not connect to the provided chain: {:?}",
				chain_spec
			)));
		};

		self._connection = Some(SuccessChainConnection(Arc::new(Mutex::new(rpc_responses))));
		self.id = Some(chain_id);

		Ok(())
	}

	pub fn id(&self) -> ChainId {
		return self.id.unwrap();
	}
}

#[maybe_async::async_impl(?Send)]
impl<P, TChain> Request for ScProvider<Arc<P>, TChain>
where
	Arc<P>: PlatformRef,
{
	async fn request_raw(&self, method: &str, params: RpcParams) -> Result<String> {
		let mut guarded_client = self.inner.lock().unwrap();
		match &self._connection {
			Some(SuccessChainConnection(json_rpc_responses)) => {
				let payload = to_json_req(method, params).unwrap();

				guarded_client
					.json_rpc_request(payload, self.id.unwrap())
					.map_err(|e| Error::Client(Box::new(e)))
					.unwrap();

				let mut guarded_responses = json_rpc_responses.lock().unwrap();
				let next_data = guarded_responses.next().await.unwrap();
				return Ok(next_data);
			},
			None => return Err(Error::ConnectionClosed),
		}
	}

	async fn request<R: DeserializeOwned + Debug>(
		&self,
		method: &str,
		params: RpcParams,
	) -> Result<R> {
		let raw_response = self.request_raw(method, params).await.unwrap();
		let parsed_response = serde_json::from_str::<JsonRpcResponse<R>>(&raw_response).unwrap();
		match parsed_response.result {
			Some(data) => return Ok(data),
			None => return Err(Error::JsonRpcError(parsed_response.error.unwrap_or_default())),
		}
	}
}

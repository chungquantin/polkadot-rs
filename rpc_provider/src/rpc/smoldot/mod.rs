//! Reference: https://github.com/smol-dot/smoldot/blob/main/light-base/examples/basic.rs
use crate::{error::RpcError, primitives::RpcParams, to_json_req, Error, Request, Result};
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
pub struct SuccessChainConnection<P: PlatformRef> {
	chain_id: ChainId,
	json_rpc_responses: Arc<Mutex<JsonRpcResponses<P>>>,
}

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

#[maybe_async::async_impl(?Send)]
impl<P, TChain> Request for SmoldotLightClient<Arc<P>, TChain>
where
	Arc<P>: PlatformRef,
{
	async fn request_raw(&self, method: &str, params: RpcParams) -> Result<String> {
		let mut guarded_client = self.inner.lock().unwrap();
		match &self._current_conn {
			Some(SuccessChainConnection { chain_id, json_rpc_responses }) => {
				let payload = to_json_req(method, params).unwrap();

				guarded_client
					.json_rpc_request(payload, *chain_id)
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

#[derive(Clone)]
pub struct SmoldotLightClient<P, TChain>
where
	P: PlatformRef,
{
	inner: Arc<Mutex<smoldot_light::Client<P, TChain>>>,
	_current_conn: Option<SuccessChainConnection<P>>,
}

impl<P> SmoldotLightClient<Arc<P>, ()>
where
	Arc<P>: PlatformRef,
{
	/// Default platform is a "plug and play" platfrom use environments of your device
	pub fn with_default_platform() -> SmoldotLightClient<Arc<DefaultPlatform>, ()> {
		let (client_name, client_version) =
			(env!("CARGO_PKG_NAME").into(), env!("CARGO_PKG_VERSION").into());
		let platfrom = DefaultPlatform::new(client_name, client_version);
		let client = smoldot_light::Client::new(platfrom);
		SmoldotLightClient { inner: Arc::new(Mutex::new(client)), _current_conn: None }
	}

	pub fn connect_relaychain(
		&self,
		chain_spec: &'static str,
	) -> Result<SuccessChainConnection<Arc<P>>> {
		self.connect_chain(chain_spec, (), Some(vec![]))
	}
}

impl<P, TChain> SmoldotLightClient<Arc<P>, TChain>
where
	Arc<P>: PlatformRef,
{
	/// Any advance usage will likely require a custom implementation of these bindings.
	pub fn new_with_platform(platform: P) -> Self {
		let platform = Arc::new(platform);
		let client = smoldot_light::Client::new(platform);
		return SmoldotLightClient { inner: Arc::new(Mutex::new(client)), _current_conn: None };
	}

	/// Connect to Substrate-based blockchain
	/// - `relay_chains``: If the chain spec is for the parachain, we need to add the relaychain that parachain relies on
	/// Because the `Client` might contain multiple different chains whose similar identifier
	pub fn connect_chain(
		&self,
		chain_spec: &'static str,
		user_data: TChain,
		relay_chains: Option<Vec<ChainId>>,
	) -> Result<SuccessChainConnection<Arc<P>>> {
		// Ask the client to connect to Polkadot.
		let mut guarded_client = self.inner.lock().unwrap();
		let add_chain_success = guarded_client
			.add_chain(smoldot_light::AddChainConfig {
				specification: chain_spec,
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

		Ok(SuccessChainConnection {
			chain_id,
			json_rpc_responses: Arc::new(Mutex::new(rpc_responses)),
		})
	}

	pub fn use_connection(
		&mut self,
		chain_connection: SuccessChainConnection<Arc<P>>,
	) -> &mut Self {
		self._current_conn = Some(chain_connection);
		self
	}
}

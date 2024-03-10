use crate::{Request, Result};

pub trait ProviderInterface: Request {
	#[allow(async_fn_in_trait)]
	async fn connect(&mut self) -> Result<()>;

	#[allow(async_fn_in_trait)]
	async fn disconnect(&mut self) -> Result<()>;
}

mod jsonrpsee_types {
	use crate::primitives::RpcParams;
	use jsonrpsee::core::traits::ToRpcParams;
	use serde_json::value::RawValue;

	pub struct RpcParamsWrapper(pub RpcParams);

	impl ToRpcParams for RpcParamsWrapper {
		fn to_rpc_params(self) -> core::result::Result<Option<Box<RawValue>>, serde_json::Error> {
			if let Some(json) = self.0.build() {
				RawValue::from_string(json).map(Some)
			} else {
				Ok(None)
			}
		}
	}
}

pub use jsonrpsee_types::*;

use crate::{Request, Result};

pub trait ProviderInterface: Request {
	#[allow(async_fn_in_trait)]
	async fn connect(&mut self) -> Result<()>;

	#[allow(async_fn_in_trait)]
	async fn disconnect(&mut self) -> Result<()>;
}

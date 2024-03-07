use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ChainSpecMetadata {
	pub id: String,
	pub name: String,
	pub chainType: String,
	pub protocolId: Option<String>,
}

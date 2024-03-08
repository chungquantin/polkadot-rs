use core::fmt::Debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcError {
	code: i128,
	message: String,
}

impl Default for RpcError {
	fn default() -> Self {
		return Self { code: i128::default(), message: "No matched rpc error".to_string() };
	}
}

#[derive(Debug)]
pub enum Error {
	JsonRpcError(RpcError),
	SerdeJson(serde_json::error::Error),
	ExtrinsicFailed(String),
	MpscSend(String),
	InvalidUrl(String),
	InvalidChainSpec(String),
	RecvError(String),
	Io(String),
	MaxConnectionAttemptsExceeded,
	ConnectionClosed,
	Client(Box<dyn Debug + Send + Sync + 'static>),
}

impl From<serde_json::error::Error> for Error {
	fn from(error: serde_json::error::Error) -> Self {
		Self::SerdeJson(error)
	}
}

use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
#[allow(unused_imports)]
pub use std_only::*;
#[cfg(feature = "std")]
mod std_only {
	use super::*;
	use std::sync::mpsc::{RecvError, SendError};

	impl From<SendError<String>> for Error {
		fn from(error: SendError<String>) -> Self {
			Self::MpscSend(error.0)
		}
	}

	impl From<RecvError> for Error {
		fn from(error: RecvError) -> Self {
			Self::RecvError(format!("{error:?}"))
		}
	}

	impl From<std::io::Error> for Error {
		fn from(error: std::io::Error) -> Self {
			Self::Io(format!("{error:?}"))
		}
	}

	impl From<url::ParseError> for Error {
		fn from(error: url::ParseError) -> Self {
			Self::InvalidUrl(format!("{error:?}"))
		}
	}
}

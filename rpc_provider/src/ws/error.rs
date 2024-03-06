use thiserror::Error;

#[derive(Error, Debug)]
pub enum WsProviderError {
    #[error("Normal Closure")]
    E1000,
    #[error("Going Away")]
    E1001,
    #[error("Protocol Error")]
    E1002,
    #[error("Unsupported Data")]
    E1003,
    #[error("(For future)")]
    E1004,
    #[error("No Status Received")]
    E1005,
    #[error("Abnormal Closure")]
    E1006,
    #[error("Invalid frame payload data")]
    E1007,
    #[error("Policy Violation")]
    E1008,
    #[error("Message too big")]
    E1009,
    #[error("Missing Extension")]
    E1010,
    #[error("Internal Error")]
    E1011,
    #[error("Service Restart")]
    E1012,
    #[error("Try Again Later")]
    E1013,
    #[error("Bad Gateway")]
    E1014,
    #[error("TLS Handshake")]
    E1015,
    #[error("WsProviderError: `{0}`")]
    Other(String),
}

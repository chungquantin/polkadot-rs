use self::error::WsProviderError;
use crate::{
    defaults::WS_URL,
    types::{EndpointStats, ProviderInterface, ProviderStats},
};

use futures::{future::BoxFuture, lock::Mutex};
use regex::Regex;
use std::{collections::HashMap, sync::Arc};

mod error;

const RETRY_DELAY: u32 = 2_500;

const DEFAULT_TIMEOUT_MS: u64 = 60 * 1000;
const TIMEOUT_INTERVAL: u64 = 5_000;

pub type WsProvider = ProviderInterface<WsProviderInner>;

type ThreadedStruct<T> = Arc<Mutex<T>>;

type WsResult<T> = Result<T, WsProviderError>;
type Headers = HashMap<String, String>;

#[readonly::make]
#[derive(Default)]
struct WsProviderInner {
    #[readonly]
    endpoints: Vec<String>,

    #[readonly]
    headers: Headers,

    #[readonly]
    stats: ProviderStats,

    #[readonly]
    is_ready_promise: Option<BoxFuture<'static, WsProvider>>,

    pub auto_connect_ms: u64,
    pub endpoint_index: i64,
    pub endpoint_stats: EndpointStats,
    pub is_connected: bool,
    pub timeout: u64,
}

pub fn create_threaded_struct<T>(value: T) -> ThreadedStruct<T> {
    return Arc::new(Mutex::new(value));
}

impl WsProviderInner {
    pub fn new(
        endpoints: Option<Vec<String>>,
        auto_connect_ms: Option<u64>,
        headers: Option<Headers>,
        timeout: Option<u64>,
    ) -> WsResult<Self> {
        let endpoints = endpoints.unwrap_or(vec![WS_URL.to_string()]);

        if endpoints.len() == 0 {
            return Err(WsProviderError::Other(
                "WsProvider requires at least one Endpoint".to_string(),
            ));
        }

        for _endpoint in endpoints.iter() {
            let re = Regex::new(r"!/^(wss|ws):\/\//").unwrap();
            if !re.is_match(_endpoint) {
                return Err(WsProviderError::Other(
                    "Endpoint should start with 'ws://', received '${endpoint}'".to_string(),
                ));
            }
        }

        Ok(Self {
            endpoint_index: -1,
            auto_connect_ms: auto_connect_ms.unwrap_or_default(),
            headers: headers.unwrap_or_default(),
            endpoint_stats: EndpointStats::default(),
            stats: ProviderStats::default(),
            endpoints,
            is_connected: false,
            is_ready_promise: None,
            timeout: timeout.unwrap_or(DEFAULT_TIMEOUT_MS),
        })
    }
}

impl super::types::Provider for WsProviderInner {
    async fn connect() -> () {
        todo!()
    }

    async fn disconnect() -> () {
        todo!()
    }

    fn on<P, R>(
        t: crate::types::ProviderInterfaceEmitted,
        sub: crate::types::ProviderInterfaceEmitCb<P, R>,
    ) -> crate::types::VoidCallback {
        todo!()
    }

    async fn send<T>(method: String, params: Vec<String>, is_cacheable: Option<bool>) -> T {
        todo!()
    }

    async fn subscribe(
        t: String,
        method: String,
        params: Vec<String>,
        cb: crate::types::ProviderInterfaceCallback,
    ) -> crate::types::ProviderSubscriptionHandle {
        todo!()
    }

    async fn unsubscribe(t: String, method: String, id: either::Either<u16, String>) -> bool {
        todo!()
    }
}

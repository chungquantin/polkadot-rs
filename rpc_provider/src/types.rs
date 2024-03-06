use anyhow::Result;
use either::Either;

#[derive(Default)]
pub struct ProviderInterface<P>
where
    P: Provider + Default,
{
    /** true if the provider supports subscriptions (not available for HTTP) */
    has_subscriptions: bool,
    /** true if the clone() functionality is available on the provider */
    is_clonable: bool,
    /** true if the provider is currently connected (ws/sc has connection logic) */
    is_connected: bool,
    /** (optional) stats for the provider with connections/bytes */
    stats: Option<ProviderStats>,
    inner: P,
}

pub(crate) type VoidCallback = Box<dyn FnOnce() -> ()>;
pub(crate) type ProviderInterfaceCallback = Box<dyn FnOnce() -> Result<()>>;
pub(crate) type ProviderInterfaceEmitCb<P, R> = Box<dyn FnOnce(Option<P>) -> Result<R>>;
pub(crate) enum ProviderInterfaceEmitted {
    Connected,
    Disconnected,
    Error,
}
pub(crate) type ProviderSubscriptionHandle = Either<u64, String>;

pub(crate) trait Provider {
    async fn connect() -> ();
    async fn disconnect() -> ();
    fn on<P, R>(t: ProviderInterfaceEmitted, sub: ProviderInterfaceEmitCb<P, R>) -> VoidCallback;
    async fn send<T>(method: String, params: Vec<String>, is_cacheable: Option<bool>) -> T;
    async fn subscribe(
        t: String,
        method: String,
        params: Vec<String>,
        cb: ProviderInterfaceCallback,
    ) -> ProviderSubscriptionHandle;
    async fn unsubscribe(t: String, method: String, id: Either<u16, String>) -> bool;
}

#[derive(Default)]
pub(crate) struct EndpointStats {
    /// The total number of bytes sent
    bytes_recv: u64,
    /// The total number of bytes received
    bytes_sent: u64,
    /// The number of cached/in-progress requests made
    cached: u64,
    /// The number of errors found
    errors: u64,
    /// The number of requests
    requests: u64,
    /// The number of subscriptions
    subscriptions: u64,
    /// The number of request timeouts
    timeout: u64,
}

/** Overall stats for the provider */
#[derive(Default)]
pub(crate) struct ProviderStats {
    /** Details for the active/open requests */
    pub active: ProviderActiveStats,
    /** The total requests that have been made */
    pub total: EndpointStats,
}

#[derive(Default)]
pub(crate) struct ProviderActiveStats {
    /** Number of active requests */
    requests: u64,
    /** Number of active subscriptions */
    subscriptions: u64,
}

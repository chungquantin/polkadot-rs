use crate::{Error, HandleSubscription, Result};
use jsonrpsee::core::client::Subscription;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct SubscriptionWrapper<Notification> {
    inner: Subscription<Notification>,
}

#[maybe_async::async_impl(?Send)]
impl<Notification: DeserializeOwned> HandleSubscription<Notification>
    for SubscriptionWrapper<Notification>
{
    async fn next(&mut self) -> Option<Result<Notification>> {
        self.inner
            .next()
            .await
            .map(|result| result.map_err(|e| Error::Client(Box::new(e))))
    }

    async fn unsubscribe(self) -> Result<()> {
        self.inner
            .unsubscribe()
            .await
            .map_err(|e| Error::Client(Box::new(e)))
    }
}

impl<Notification> From<Subscription<Notification>> for SubscriptionWrapper<Notification> {
    fn from(inner: Subscription<Notification>) -> Self {
        Self { inner }
    }
}

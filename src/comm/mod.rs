use crate::{Event, Result, Sender};
use async_trait::async_trait;
use dyn_clonable::clonable;
use std::fmt::Debug;

mod http;
mod http_webhook;
mod ws;
mod ws_reverse;

pub use http::HTTP;
pub use http_webhook::HTTPWebHook;
pub use ws::WebSocket;
pub use ws_reverse::WebSocketReverse;

#[async_trait]
#[clonable]
pub trait Comm: Clone + Debug + Send + Sync {
    async fn start(
        &self,
        action_receiver: Sender<String>,
        event_receiver: Sender<Event>,
    ) -> Result<()>;
}

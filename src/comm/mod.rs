use crate::{Action, Event, Result, Sender};
use async_trait::async_trait;
use dyn_clonable::clonable;
use std::{collections::HashMap, fmt::Debug};

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
        action_handlers: HashMap<String, Action>,
        event_receiver: Sender<Event>,
        platform: String,
    ) -> Result<()>;
}

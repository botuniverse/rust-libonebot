use crate::{config::ConfigFileCommMethod, Action, Error, Event, Result, Sender};
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

pub(crate) fn from_config_file_comm_method(
    comm_method: &ConfigFileCommMethod,
) -> Result<Box<dyn Comm>> {
    match comm_method.r#type.as_str() {
        "http" => HTTP::from_config_file_comm_method(comm_method),
        "http_webhook" => HTTPWebHook::from_config_file_comm_method(comm_method),
        "ws" => WebSocket::from_config_file_comm_method(comm_method),
        "ws_reverse" => WebSocketReverse::from_config_file_comm_method(comm_method),
        _ => Err(Error::msg("config error: unsupport communication type")),
    }
}

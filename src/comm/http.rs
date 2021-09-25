use crate::{Action, Comm, Event, Result};
use async_trait::async_trait;
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::broadcast::Sender;
use warp::Filter;

#[derive(Debug, Clone)]
pub struct HTTP {
    pub socket_addr: SocketAddr,
}

impl HTTP {
    pub fn new(socket_addr: SocketAddr) -> Self {
        Self { socket_addr }
    }
}

#[async_trait]
impl Comm for HTTP {
    async fn start(
        &self,
        action_handlers: HashMap<String, Action>,
        _event_sender: Sender<Event>,
        _platform: String,
    ) -> Result<()> {
        let handler = warp::post()
            .and(warp::body::bytes())
            .map(move |b: bytes::Bytes| {
                let action_json = serde_json::from_str::<crate::action::ActionJson>(
                    &std::str::from_utf8(&b).unwrap().to_owned(),
                )
                .unwrap();
                let action = action_handlers.get(&action_json.action).unwrap();
                (action.action)(action_json.params)
            });

        warp::serve(handler).run(self.socket_addr).await;

        Ok(())
    }
}

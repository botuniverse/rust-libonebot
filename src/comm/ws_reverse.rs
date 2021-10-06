use crate::{config::ConfigFileCommMethod, Action, Comm, Event, Result};
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use std::{collections::HashMap, fmt::Display};
use tokio::sync::broadcast::Sender;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

#[derive(Debug, Clone)]
pub struct WebSocketReverse {
    connect_url: String,
}

impl WebSocketReverse {
    pub fn new<S: Display>(connect_url: S) -> Self {
        Self {
            connect_url: connect_url.to_string(),
        }
    }

    pub(crate) fn from_config_file_comm_method(
        comm_method: &ConfigFileCommMethod,
    ) -> Result<Box<dyn Comm>> {
        let ws_reverse = Self::new(
            comm_method
                .url
                .clone()
                .unwrap_or_else(|| "127.0.0.1:5700".to_string()),
        );
        Ok(Box::new(ws_reverse))
    }
}

#[async_trait]
impl Comm for WebSocketReverse {
    async fn start(
        &self,
        action_handlers: HashMap<String, Action>,
        event_sender: Sender<Event>,
        platform: String,
    ) -> Result<()> {
        let mut event_receiver = event_sender.subscribe();

        let (ws_stream, _) = tokio_tungstenite::connect_async(&self.connect_url).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        loop {
            tokio::select! {
                event = event_receiver.recv() => {
                    if let Ok(mut event) = event {
                        event = event.platform(&platform);
                        ws_sender
                            .send(TungsteniteMessage::Text(event.to_json().unwrap()))
                            .await
                            .unwrap();
                    }
                }
                msg = ws_receiver.next() => {
                    if let Some(Ok(msg)) = msg {
                        if let Ok(text) = msg.into_text() {
                            let action_json =
                                serde_json::from_str::<crate::action::ActionJson>(
                                    &text,
                                )
                                .unwrap();
                            let action =
                                action_handlers.get(&action_json.action).unwrap();
                            ws_sender
                                .send(TungsteniteMessage::Text((action.action)(
                                    action_json.params,
                                )))
                                .await
                                .unwrap();
                        }
                    }
                }
            }
        }
    }
}

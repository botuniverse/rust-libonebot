use crate::{Comm, Event, Result};
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

#[derive(Debug, Clone)]
pub struct WebSocketReverse {
    connect_url: String,
}

#[async_trait]
impl Comm for WebSocketReverse {
    async fn start(
        &self,
        action_sender: Sender<String>,
        event_sender: Sender<Event>,
    ) -> Result<()> {
        let mut event_receiver = event_sender.subscribe();

        let (ws_stream, _) = tokio_tungstenite::connect_async(&self.connect_url).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        tokio::spawn(async move {
            loop {
                let event = event_receiver.recv().await;
                if let Ok(event) = event {
                    ws_sender
                        .send(TungsteniteMessage::Text(event.to_json().unwrap()))
                        .await
                        .unwrap();
                }
            }
        });

        loop {
            let msg = ws_receiver.next().await;
            if let Some(msg) = msg {
                if let Ok(msg) = msg {
                    if let Ok(text) = msg.into_text() {
                        action_sender.send(text).unwrap();
                    }
                }
            }
        }
    }
}

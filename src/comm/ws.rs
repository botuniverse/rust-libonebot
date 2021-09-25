use crate::{Action, Comm, Event, Result};
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use std::{collections::HashMap, net::SocketAddr};
use tokio::{net::TcpListener, sync::broadcast::Sender};
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

#[derive(Debug, Clone)]
pub struct WebSocket {
    socket_addr: SocketAddr,
}

impl WebSocket {
    pub fn new(socket_addr: SocketAddr) -> Self {
        Self { socket_addr }
    }
}

#[async_trait]
impl Comm for WebSocket {
    async fn start(
        &self,
        action_handlers: HashMap<String, Action>,
        event_sender: Sender<Event>,
        platform: String,
    ) -> Result<()> {
        let socket_addr = self.socket_addr.clone();

        let listener = TcpListener::bind(&socket_addr).await?;

        while let Ok((stream, _)) = listener.accept().await {
            let mut event_receiver = event_sender.subscribe();
            let platform = platform.clone();
            let action_handlers = action_handlers.clone();
            tokio::spawn(async move {
                let ws_stream = tokio_tungstenite::accept_async(stream).await.unwrap();
                let (mut ws_sender, mut ws_receiver) = ws_stream.split();

                loop {
                    tokio::select! {
                        event = event_receiver.recv() => {
                            if let Ok(mut event) = event {
                                event.platform = platform.clone();
                                ws_sender
                                    .send(TungsteniteMessage::Text(event.to_json().unwrap()))
                                    .await
                                    .unwrap();
                            }
                        }
                        msg = ws_receiver.next() => {
                            if let Some(msg) = msg {
                                if let Ok(msg) = msg {
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
            });
        }

        Ok(())
    }
}

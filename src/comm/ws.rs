use crate::{config::ConfigFileCommMethod, Action, Comm, Error, Event, Result};
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use std::{
    collections::HashMap,
    net::{SocketAddr, ToSocketAddrs},
};
use tokio::{net::TcpListener, sync::broadcast::Sender};
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

#[derive(Debug, Clone)]
pub struct WebSocket {
    socket_addr: SocketAddr,
}

impl WebSocket {
    pub fn new<A: ToSocketAddrs>(socket_addr: A) -> Result<Self> {
        let mut addrs = socket_addr.to_socket_addrs()?;
        if let Some(addr) = addrs.next() {
            if addrs.next() == None {
                return Ok(Self { socket_addr: addr });
            }
        };
        Err(Error::msg(format!(
            "communication error: except 1 socket address but found {}",
            addrs.count()
        )))
    }

    pub(crate) fn from_config_file_comm_method(
        comm_method: &ConfigFileCommMethod,
    ) -> Result<Box<dyn Comm>> {
        Ok(Box::new(Self::new(format!(
            "{}:{}",
            comm_method.host.clone().unwrap_or("127.0.0.1".to_string()),
            comm_method.port.unwrap_or(5700)
        ))?))
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

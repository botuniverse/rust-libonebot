use crate::{Comm, Event, Result};
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast::{Receiver, Sender},
};
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

#[derive(Debug, Clone)]
pub struct WebSocket {
    socket_addr: SocketAddr,
}

impl WebSocket {
    pub fn new(socket_addr: SocketAddr) -> Self {
        Self { socket_addr }
    }

    async fn accept_connection(
        _: SocketAddr,
        stream: TcpStream,
        action_sender: Sender<String>,
        mut event_receiver: Receiver<Event>,
    ) -> Result<()> {
        let ws_stream = tokio_tungstenite::accept_async(stream).await?;
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

#[async_trait]
impl Comm for WebSocket {
    async fn start(
        &self,
        action_sender: Sender<String>,
        event_sender: Sender<Event>,
    ) -> Result<()> {
        let socket_addr = self.socket_addr.clone();

        let listener = TcpListener::bind(&socket_addr).await?;

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream.peer_addr()?;
            let action_sender = action_sender.clone();
            let event_receiver = event_sender.subscribe();
            tokio::spawn(Self::accept_connection(
                peer,
                stream,
                action_sender,
                event_receiver,
            ));
        }

        Ok(())
    }
}

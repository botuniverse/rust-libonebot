use crate::{Communication, Event, Result};
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast::{Receiver, Sender},
};
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

#[derive(Clone)]
pub struct WebSocket {
    socket_addr: SocketAddr,
    route: String,
    access_token: String,
}

#[async_trait]
impl Communication for WebSocket {
    async fn start(
        &self,
        action_sender: Sender<String>,
        event_sender: Sender<Event>,
    ) -> Result<()> {
        let socket_addr = self.socket_addr.clone();
        let route = self.route.clone();

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

impl WebSocket {
    async fn accept_connection(
        peer: SocketAddr,
        stream: TcpStream,
        action_sender: Sender<String>,
        mut event_receiver: Receiver<Event>,
    ) -> Result<()> {
        let ws_stream = tokio_tungstenite::accept_async(stream).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        loop {
            tokio::select! {
                event = event_receiver.recv() => {
                    if let Ok(event) = event {
                        ws_sender.send(TungsteniteMessage::Text(event.to_json())).await?;
                    }
                }
                msg = ws_receiver.next() => {
                    if let Some(msg) = msg {
                        if let Ok(msg) = msg {
                            if let Ok(text) = msg.into_text() {
                                action_sender.send(text);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

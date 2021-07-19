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
    async fn start(&self, event_tx: Sender<Event>) -> Result<()> {
        let socket_addr = self.socket_addr.clone();
        let route = self.route.clone();

        let listener = TcpListener::bind(&socket_addr).await?;

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream.peer_addr()?;
            let mut event_rx = event_tx.subscribe();
            tokio::spawn(Self::accept_connection(peer, stream, event_rx));
        }

        Ok(())
    }

    async fn push_event(&self, event: Event) -> Result<()> {
        Ok(())
    }
}

impl WebSocket {
    async fn accept_connection(
        peer: SocketAddr,
        stream: TcpStream,
        mut event_rx: Receiver<Event>,
    ) -> Result<()> {
        let mut ws_stream = tokio_tungstenite::accept_async(stream).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        loop {
            tokio::select! {
                event = event_rx.recv() => {
                    ws_sender.send(TungsteniteMessage::Text("received an event".to_string())).await?;
                }
                msg = ws_receiver.next() => {
                    match msg {
                        Some(msg) => {
                            let msg = msg?;
                            if msg.is_close() {
                                break;
                            }
                        },
                        None => break,
                    }
                }
            }
        }

        Ok(())
    }
}

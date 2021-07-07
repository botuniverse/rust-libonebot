use crate::{Communication, Event, Result};
use async_trait::async_trait;
use futures_util::StreamExt;
use std::net::SocketAddr;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::Sender,
};

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
            tokio::spawn(Self::accept_connection(peer, stream));
        }

        Ok(())
    }

    async fn push_event(&self, event: Event) -> Result<()> {
        Ok(())
    }
}

impl WebSocket {
    async fn accept_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
        let mut ws_stream = tokio_tungstenite::accept_async(stream).await?;
        let (ws_sender, mut ws_receiver) = ws_stream.split();

        while let Some(msg) = ws_receiver.next().await {
            let msg = msg?;
        }

        Ok(())
    }
}

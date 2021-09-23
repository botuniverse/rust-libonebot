use crate::{Comm, Event, Result};
use async_trait::async_trait;
use std::net::SocketAddr;
use tokio::sync::broadcast::{Receiver, Sender};
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
        action_sender: Sender<String>,
        _event_sender: Sender<Event>,
    ) -> Result<()> {
        let handler = warp::post()
            .and(warp::body::bytes())
            .map(move |b: bytes::Bytes| {
                action_sender
                    .send(std::str::from_utf8(&b).unwrap().to_owned())
                    .unwrap();
                ""
            });

        warp::serve(handler).run(self.socket_addr).await;

        Ok(())
    }
}

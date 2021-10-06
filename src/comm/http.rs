use crate::{config::ConfigFileCommMethod, Action, Comm, Error, Event, Result};
use async_trait::async_trait;
use std::{
    collections::HashMap,
    net::{SocketAddr, ToSocketAddrs},
};
use tokio::sync::broadcast::Sender;
use warp::Filter;

#[derive(Debug, Clone)]
pub struct HTTP {
    pub socket_addr: SocketAddr,
}

impl HTTP {
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
            comm_method
                .host
                .clone()
                .unwrap_or_else(|| "127.0.0.1".to_string()),
            comm_method.port.unwrap_or(5700)
        ))?))
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

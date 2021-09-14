use config::Config;
use serde::Serialize;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};
use thiserror::Error;

pub use tokio::sync::broadcast::{Receiver, Sender};

pub struct OneBot {
    pub platform: String,
    pub config: Config,
    pub logger: Logger,

    event_generator: Box<dyn Fn(Sender<Event>) -> Result<()>>,
    action_handlers: HashMap<String, Action>,

    comms: Vec<Box<dyn Comm>>,
    action_sender: Sender<String>,
    _action_default_receiver: Receiver<String>,
    event_sender: Sender<Event>,
    _event_default_receiver: Receiver<Event>,

    pub extended: HashMap<String, String>,
}

impl OneBot {
    pub fn new<S: Display, F: ConfigFile>(platform: S, config_file: F) -> Result<Self> {
        let config = Config::from_config_file(&config_file)?;

        let mut comms: Vec<Box<dyn Comm>> = Vec::new();
        if let Some(comm_methods) = config_file.comm_methods() {
            for comm_method in comm_methods {
                match comm_method.r#type.as_str() {
                    "http" => {}
                    "http_webhook" => {}
                    "ws" => comms.push(Box::new(comm::WebSocket::new(
                        format!(
                            "{}:{}",
                            comm_method.host.clone().unwrap_or("127.0.0.1".to_string()),
                            comm_method.port.unwrap_or(80)
                        )
                        .parse()?,
                    ))),
                    "ws_reverse" => {}
                    _ => return Err(Error::msg("config error: unsupport communication type")),
                };
            }
        }

        let (action_sender, _action_default_receiver) =
            tokio::sync::broadcast::channel(config.channel_capacity.action);
        let (event_sender, _event_default_receiver) =
            tokio::sync::broadcast::channel(config.channel_capacity.event);

        Ok(Self {
            platform: platform.to_string(),
            config,
            logger: Logger {},
            event_generator: Box::new(Self::default_event_generator),
            action_handlers: HashMap::new(),
            comms,
            action_sender,
            _action_default_receiver,
            event_sender,
            _event_default_receiver,
            extended: HashMap::new(),
        })
    }

    pub fn add_comm<C: 'static + Comm>(&mut self, comm: C) {
        self.comms.push(Box::new(comm));
    }

    pub async fn run(&mut self) {
        for comm in self.comms.iter() {
            let comm = comm.clone();
            let action_sender = self.action_sender.clone();
            let event_sender = self.event_sender.clone();
            tokio::spawn(async move {
                comm.start(action_sender, event_sender).await.unwrap();
            });
        }

        let mut action_receiver = self.action_sender.subscribe();
        let action_handlers = self.action_handlers.clone();
        tokio::spawn(async move {
            loop {
                let action_json = action_receiver.recv().await.unwrap();
                let action = action_handlers
                    .get(
                        &serde_json::from_str::<action::ActionJson>(&action_json)
                            .unwrap()
                            .action,
                    )
                    .unwrap();
                (action.action)(HashMap::new());
            }
        });

        // logger: "start"

        (self.event_generator)(self.event_sender.clone()).unwrap();

        // logger: "shutdown"
    }

    pub fn shutdown(&self) {}

    pub fn register_event_generator<F: 'static + Fn(Sender<Event>) -> Result<()>>(
        &mut self,
        event_generator: F,
    ) {
        self.event_generator = Box::new(event_generator);
    }

    fn default_event_generator(_: Sender<Event>) -> Result<()> {
        Ok(())
    }

    pub fn register_action_handler(&mut self, name: String, action: fn(_: HashMap<&str, String>)) {
        self.action_handlers.insert(name, Action::from(action));
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct User {
    id: String,
    username: String,

    nickname: String,
    display_name: String,

    extended: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Group {
    id: i64,

    name: String,

    extended: HashMap<String, String>,
}

pub mod action;
pub use action::Action;

pub mod comm;
pub use comm::Comm;

pub mod config;
pub use config::ConfigFile;

pub mod event;
pub use event::{Event, EventContent};

pub mod log;
pub use log::Logger;

pub mod message;
pub use message::{Message, MessageSegment};

pub use anyhow::{Error, Result};

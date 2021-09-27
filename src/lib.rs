use config::Config;
use serde::Serialize;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};
use thiserror::Error;

pub use tokio::sync::broadcast::{Receiver, Sender};

static DEFAULT_CHANNEL_CAPACITY: usize = 128 as usize;

pub struct OneBot {
    platform: String,
    config: Config,
    logger: Logger,

    event_generator: Box<dyn Fn(Sender<Event>) -> Result<()>>,
    action_handlers: HashMap<String, Action>,

    comms: HashMap<String, Box<dyn Comm>>,

    event_sender: Sender<Event>,
    _event_default_receiver: Receiver<Event>,
}

impl OneBot {
    pub fn new<S: Display>(platform: S) -> Self {
        let (event_sender, _event_default_receiver) =
            tokio::sync::broadcast::channel(DEFAULT_CHANNEL_CAPACITY);

        Self {
            platform: platform.to_string(),
            config: Config::new(),
            logger: Logger::new(),
            event_generator: Box::new(Self::default_event_generator),
            action_handlers: HashMap::new(),
            comms: HashMap::new(),
            event_sender,
            _event_default_receiver,
        }
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    fn add_comm_box<S: Display>(mut self, name: &S, comm: Box<dyn Comm>) -> Self {
        self.comms.insert(name.to_string(), comm);
        self
    }

    pub fn add_comm<S: Display, C: 'static + Comm>(self, name: &S, comm: C) -> Self {
        self.add_comm_box(&name, Box::new(comm))
    }

    pub fn init_from_file<F: ConfigFile>(mut self, config_file: F) -> Result<Self> {
        self = self.config(Config::from_config_file(&config_file)?);

        self.comms = HashMap::new();

        if let Some(comm_methods) = config_file.comm_methods() {
            for (comm_name, comm_method) in comm_methods {
                self =
                    self.add_comm_box(&comm_name, comm::from_config_file_comm_method(comm_method)?);
            }
        }

        Ok(self)
    }

    pub async fn run(&mut self) {
        for (_, comm) in self.comms.iter() {
            let comm = comm.clone();
            let action_handlers = self.action_handlers.clone();
            let event_sender = self.event_sender.clone();
            let platform = self.platform.clone();
            tokio::spawn(async move {
                comm.start(action_handlers, event_sender, platform.clone())
                    .await
                    .unwrap();
            });
        }

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
        loop {}
    }

    pub fn register_action_handler<S: Display>(
        &mut self,
        name: S,
        action: fn(_: serde_json::Value) -> String,
    ) {
        self.action_handlers
            .insert(name.to_string(), Action::from(action));
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct User {
    id: String,
    pub username: String,

    pub nickname: String,
    pub display_name: String,

    extended: HashMap<String, String>,
}

impl User {
    pub fn new<S: Display>(id: S) -> Self {
        Self {
            id: id.to_string(),
            username: String::new(),
            nickname: String::new(),
            display_name: String::new(),
            extended: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    id: String,

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

pub mod logger;
pub use logger::Logger;

pub mod message;
pub use message::{Message, MessageSegment};

pub use anyhow::{Error, Result};

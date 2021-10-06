use config::Config;
use serde::Serialize;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};
// use thiserror::Error;
use tokio::sync::broadcast::{Receiver, Sender};

pub struct OneBot {
    platform: String,
    self_user: Option<User>,
    config: Option<Config>,

    event_generator: Box<dyn Fn(Sender<Event>) -> Result<()>>,
    action_handlers: HashMap<String, Action>,

    comms: HashMap<String, Box<dyn Comm>>,

    event_sender: Sender<Event>,
    _event_default_receiver: Receiver<Event>,
}

impl OneBot {
    pub fn new<S: Display>(platform: S) -> Self {
        let (event_sender, _event_default_receiver) = tokio::sync::broadcast::channel(1);

        Self {
            platform: platform.to_string(),
            self_user: None,
            config: None,
            event_generator: Box::new(Self::default_event_generator),
            action_handlers: HashMap::new(),
            comms: HashMap::new(),
            event_sender,
            _event_default_receiver,
        }
    }

    pub fn platform(&self) -> String {
        self.platform.clone()
    }

    pub fn set_platform<S: Display>(&mut self, new_platform: &S) {
        self.platform = new_platform.to_string();
    }

    pub fn has_platform(&self) -> bool {
        !self.platform.is_empty()
    }

    pub fn self_user(&self) -> Option<User> {
        self.self_user.clone()
    }

    pub fn set_self_user(&mut self, user: User) {
        self.self_user = Some(user);
    }

    pub fn has_self_user(&self) -> bool {
        self.self_user.is_some()
    }

    pub fn has_self_id(&self) -> bool {
        match &self.self_user {
            Some(user) => !user.id.is_empty(),
            None => false,
        }
    }

    pub fn config(&self) -> Option<Config> {
        self.config.clone()
    }

    pub fn set_config(&mut self, new_config: Config) {
        self.config = Some(new_config);
    }

    pub fn has_config(&self) -> bool {
        self.config.is_some()
    }

    fn add_comm_box<S: Display>(&mut self, name: &S, comm: Box<dyn Comm>) {
        self.comms.insert(name.to_string(), comm);
    }

    pub fn add_comm<S: Display, C: 'static + Comm>(&mut self, name: &S, comm: C) {
        self.add_comm_box(name, Box::new(comm));
    }

    pub fn init_from_file<F: ConfigFile>(&mut self, config_file: F) -> Result<()> {
        self.set_config(Config::from_config_file(&config_file)?);

        self.comms = HashMap::new();

        if let Some(comm_methods) = config_file.comm_methods() {
            for (comm_name, comm_method) in comm_methods {
                self.add_comm_box(&comm_name, comm::from_config_file_comm_method(comm_method)?);
            }
        }

        Ok(())
    }

    fn start_comm_methods(&self) {
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
    }

    pub async fn run(&mut self) -> Result<()> {
        if !self.has_platform() {
            let text = "必须提供 OneBot 平台名称";
            log::error!("{}", text);
            panic!("{}", text);
        }
        if !self.has_self_id() {
            let text = "必须提供 OneBot 实例对应的机器人自身 ID";
            log::error!("{}", text);
            panic!("{}", text);
        }

        let heartbeat;
        if let Some(config) = &self.config {
            heartbeat = config.heartbeat;
        } else {
            let text = "必须提供 OneBot 配置";
            log::error!("{}", text);
            panic!("{}", text);
        }

        // context.withCancel

        self.start_comm_methods();
        if let Some(_heartbeat) = heartbeat {
            self.heartbeat();
        }

        log::info!("OneBot 已启动");

        (self.event_generator)(self.event_sender.clone())?;

        log::info!("OneBot 已关闭");

        Ok(())
    }

    fn heartbeat(&self) {}

    pub fn shutdown(&self) {}

    pub fn register_event_generator<F: 'static + Fn(Sender<Event>) -> Result<()>>(
        &mut self,
        event_generator: F,
    ) {
        self.event_generator = Box::new(event_generator);
    }

    fn default_event_generator(_: Sender<Event>) -> Result<()> {
        loop {
            panic!()
        }
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

pub mod message;
pub use message::{Message, MessageSegment};

pub use anyhow::{Error, Result};

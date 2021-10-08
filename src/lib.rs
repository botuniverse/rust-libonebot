use config::Config;
use serde::Serialize;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};
// use thiserror::Error;
use tokio::sync::broadcast::{Receiver, Sender};

pub struct OneBot {
    pub platform: String,
    pub self_user: Option<User>,
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

    pub fn set_self_id<S: Display>(&mut self, id: S) -> &mut Self {
        self.self_user = Some(User::new(id));
        self
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

    pub fn set_config(&mut self, new_config: Config) -> &mut Self {
        self.config = Some(new_config);
        self
    }

    pub fn set_default_config(&mut self) -> &mut Self {
        self.config = Some(Config::new());
        self
    }

    pub fn auth(&self) -> Option<config::Auth> {
        self.config.as_ref().map(|config| config.auth.clone())
    }

    pub fn access_token(&self) -> Option<String> {
        if let Some(auth) = self.auth() {
            auth.access_token
        } else {
            None
        }
    }

    pub fn set_access_token<S: Display>(&mut self, access_token: S) -> &mut Self {
        let mut config = Config::new();
        if let Some(c) = &self.config {
            config = c.clone();
        }
        config.auth.access_token = Some(access_token.to_string());
        self.config = Some(config);
        self
    }

    pub fn has_access_token(&self) -> bool {
        if let Some(auth) = self.auth() {
            auth.access_token.is_some()
        } else {
            false
        }
    }

    pub fn heartbeat_interval(&self) -> Option<u32> {
        if let Some(config) = &self.config {
            config.heartbeat
        } else {
            None
        }
    }

    pub fn enable_heartbeat(&mut self, interval: u32) -> &mut Self {
        let mut config = Config::new();
        if let Some(c) = &self.config {
            config = c.clone();
        }
        config.heartbeat = Some(interval);
        self.config = Some(config);
        self
    }

    pub fn heartbeat_enabled(&self) -> bool {
        if let Some(config) = &self.config {
            config.heartbeat.is_some()
        } else {
            false
        }
    }

    pub fn log_to_stderr(&mut self) -> &mut Self {
        let mut config = Config::new();
        if let Some(c) = &self.config {
            config = c.clone();
        }
        config.log.output = config::LogOutput::Stderr;
        self.config = Some(config);
        self
    }

    pub fn log_to_stdout(&mut self) -> &mut Self {
        let mut config = Config::new();
        if let Some(c) = &self.config {
            config = c.clone();
        }
        config.log.output = config::LogOutput::Stdout;
        self.config = Some(config);
        self
    }

    pub fn log_to_nul(&mut self) -> &mut Self {
        let mut config = Config::new();
        if let Some(c) = &self.config {
            config = c.clone();
        }
        config.log.output = config::LogOutput::Nul;
        self.config = Some(config);
        self
    }

    pub fn log_to_path<S: Display>(&mut self, path: S) -> &mut Self {
        let mut config = Config::new();
        if let Some(c) = &self.config {
            config = c.clone();
        }
        config.log.path = Some(path.to_string());
        self.config = Some(config);
        self
    }

    pub fn set_log_level(&mut self, level: log::LevelFilter) -> &mut Self {
        let mut config = Config::new();
        if let Some(c) = &self.config {
            config = c.clone();
        }
        config.log.level = level;
        self.config = Some(config);
        self
    }

    fn add_comm_box<S: Display>(&mut self, name: &S, comm: Box<dyn Comm>) -> &mut Self {
        self.comms.insert(name.to_string(), comm);
        self
    }

    pub fn add_comm<S: Display, C: 'static + Comm>(&mut self, name: &S, comm: C) -> &mut Self {
        self.add_comm_box(name, Box::new(comm));
        self
    }

    pub fn init_from_file<F: ConfigFile>(&mut self, config_file: F) -> Result<&mut Self> {
        self.set_config(Config::from_config_file(&config_file)?);

        self.comms = HashMap::new();

        if let Some(comm_methods) = config_file.comm_methods() {
            for (comm_name, comm_method) in comm_methods {
                self.add_comm_box(&comm_name, comm::from_config_file_comm_method(comm_method)?);
            }
        }

        Ok(self)
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
        if self.platform.is_empty() {
            return Err(Error::msg("必须提供 OneBot 平台名称"));
        }
        if !self.has_self_id() {
            return Err(Error::msg("必须提供 OneBot 实例对应的机器人自身 ID"));
        }

        let heartbeat;
        if let Some(config) = &self.config {
            heartbeat = config.heartbeat;

            let mut dispatch = fern::Dispatch::new().format(|out, message, record| {
                out.finish(format_args!(
                    "{} [{}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    message
                ))
            });
            if let config::LogOutput::Stderr = config.log.output {
                dispatch = dispatch.chain(std::io::stderr());
            } else if let config::LogOutput::Stdout = config.log.output {
                dispatch = dispatch.chain(std::io::stdout());
            }
            if let Some(path) = &config.log.path {
                dispatch = dispatch.chain(fern::log_file(path)?);
            }
            dispatch.level(config.log.level).apply()?;
        } else {
            return Err(Error::msg("必须提供 OneBot 配置"));
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

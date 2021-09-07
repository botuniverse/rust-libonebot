use anyhow::Error;
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug, time::Duration};

pub use tokio::sync::broadcast::{Receiver, Sender};

pub type Result<T> = std::result::Result<T, Error>;

pub struct OneBot {
    user: User,

    config: Config,

    event_generator: Box<dyn Fn(Sender<Event>) -> Result<()>>,
    actions: HashMap<String, Action>,

    communications: Vec<Box<dyn Communication>>,
    action_sender: Sender<String>,
    action_receiver: Receiver<String>,
    event_sender: Sender<Event>,
    event_receiver: Receiver<Event>,

    pub extended: HashMap<String, String>,
}

impl OneBot {
    pub fn new(action_capacity: usize, event_capacity: usize) -> Self {
        let (action_sender, action_receiver) = tokio::sync::broadcast::channel(action_capacity);
        let (event_sender, event_receiver) = tokio::sync::broadcast::channel(event_capacity);

        Self {
            user: User::default(),
            config: Config::default(),
            event_generator: Box::new(Self::default_event_generator),
            actions: HashMap::new(),
            communications: Vec::new(),
            action_sender,
            action_receiver,
            event_sender,
            event_receiver,
            extended: HashMap::new(),
        }
    }

    pub fn add_communication<C: 'static + Communication>(&mut self, communication: C) {
        self.communications.push(Box::new(communication));
    }

    pub async fn run(&mut self) {
        for communication in self.communications.iter() {
            let communication = communication.clone();
            let action_sender = self.action_sender.clone();
            let event_sender = self.event_sender.clone();
            tokio::spawn(async move {
                communication
                    .start(action_sender, event_sender)
                    .await
                    .unwrap();
            });
        }

        let mut action_receiver = self.action_sender.subscribe();
        let actions = self.actions.clone();
        tokio::spawn(async move {
            loop {
                let action_json = action_receiver.recv().await.unwrap();
                let action = actions
                    .get(
                        &serde_json::from_str::<action::ActionJson>(&action_json)
                            .unwrap()
                            .action,
                    )
                    .unwrap();
                (action.action)(HashMap::new());
            }
        });

        (self.event_generator)(self.event_sender.clone()).unwrap();
    }

    pub fn register_event_generator<F: 'static + Fn(Sender<Event>) -> Result<()>>(
        &mut self,
        event_generator: F,
    ) {
        self.event_generator = Box::new(event_generator);
    }

    fn default_event_generator(_: Sender<Event>) -> Result<()> {
        Ok(())
    }

    pub fn register_action(&mut self, name: String, action: fn(_: HashMap<&str, String>)) {
        self.actions.insert(name, Action::from(action));
    }

    fn default_action_processor(_: String) -> Result<()> {
        Ok(())
    }
}

#[derive(Default)]
struct Config {
    rate_limit: Duration,

    pub extended: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct User {
    id: String,
    username: String,

    nickname: String,
    display_name: String,

    extended: HashMap<String, String>,
}

impl User {
    fn new(id: String) -> Self {
        Self {
            id,
            username: String::new(),
            nickname: String::new(),
            display_name: String::new(),
            extended: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    id: i64,

    name: String,

    extended: HashMap<String, String>,
}

pub mod action;
pub use action::Action;

pub mod communication;
pub use communication::Communication;

pub mod event;
pub use event::{Event, EventContent};

pub mod message;
pub use message::{Message, MessageSegment};

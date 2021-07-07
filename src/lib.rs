use anyhow::Error;
use async_trait::async_trait;
use dyn_clonable::clonable;
use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};
use tokio::sync::mpsc::{Receiver, Sender};

type Result<T> = std::result::Result<T, Error>;

pub struct Bot {
    user: User,

    config: Config,

    communications: Vec<Box<dyn Communication>>,
    event_tx: Sender<Event>,
    event_rx: Receiver<Event>,

    pub custom_field: HashMap<String, String>,
}

impl Bot {
    pub fn new(event_buffer: usize) -> Self {
        let (event_tx, event_rx) = tokio::sync::mpsc::channel(event_buffer);
        Self {
            user: User::default(),
            config: Config::default(),
            communications: Vec::new(),
            event_tx,
            event_rx,
            custom_field: HashMap::new(),
        }
    }

    pub fn add_communication(&mut self, communication: Box<dyn Communication>) {
        self.communications.push(communication);
    }

    pub async fn start(&mut self) {
        for communication in self.communications.iter() {
            let communication = communication.clone();
            let event_tx = self.event_tx.clone();
            tokio::spawn(async move {
                communication.start(event_tx).await.unwrap();
            });
        }

        self.push_events().await;
    }

    async fn push_events(&mut self) {
        while let Some(event) = self.event_rx.recv().await {
            if event.content.is_stop() {
                return;
            }
            for communication in self.communications.iter() {
                let communication = communication.clone();
                let event = event.clone();
                tokio::spawn(async move {
                    communication.push_event(event).await.unwrap();
                });
            }
        }
    }
}

#[derive(Default)]
struct Config {
    message_format: MessageFormat,
    rate_limit: Duration,

    pub custom_field: HashMap<String, String>,
}

pub enum MessageFormat {
    String,
    Array,
}

impl Default for MessageFormat {
    fn default() -> Self {
        MessageFormat::String
    }
}

#[async_trait]
#[clonable]
pub trait Communication: Clone + Send + Sync {
    async fn start(&self, event_tx: Sender<Event>) -> Result<()>;
    async fn push_event(&self, event: Event) -> Result<()>;
}

#[derive(Clone)]
pub struct Message {
    id: i64,
    source: MessageSource,
    sender: User,

    content: MessageContent,

    custom_field: HashMap<String, String>,
}

#[derive(Clone)]
pub enum MessageSource {
    Private(User),
    Group(Group),
}

#[derive(Clone)]
pub enum MessageContent {
    String(String),
    Array(Vec<MessageSegment>),
}

#[derive(Clone)]
pub enum MessageSegment {
    Text(String),
    Emoji(String),
    Image(Media),
    Record(Media),
    Video(Media),
    At(User),
    Location(f64, f64), // lat, lon
    Reply(i64),
    Foward(i64),
    Custom(HashMap<String, String>),
}

#[derive(Clone)]
pub enum Media {
    File(String),
    URL(String, bool, bool, bool), // url, cache, proxy, timeout
    Base64(String),
}

#[derive(Clone)]
pub struct Event {
    time: SystemTime,
    content: EventContent,

    custom_field: HashMap<String, String>,
}

#[derive(Clone)]
pub enum EventContent {
    Message(Message),
    Notice(Notice),
    Request(Request),
    Meta(Meta),
    Stop,
}

impl EventContent {
    fn is_stop(&self) -> bool {
        if let Self::Stop = self {
            true
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct Notice {
    custom_field: HashMap<String, String>,
}

#[derive(Clone)]
pub struct Request {
    custom_field: HashMap<String, String>,
}

#[derive(Clone)]
pub struct Meta {
    custom_field: HashMap<String, String>,
}

#[derive(Clone)]
pub struct Action {
    custom_field: HashMap<String, String>,
}

#[derive(Clone, Default)]
pub struct User {
    id: i64,
    username: String,

    nickname: String,
    display_name: String,

    custom_field: HashMap<String, String>,
}

#[derive(Clone)]
pub struct Group {
    id: i64,

    name: String,

    custom_field: HashMap<String, String>,
}

mod communications;

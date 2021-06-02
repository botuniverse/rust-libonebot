use anyhow::Error as AnyhowError;
use async_trait::async_trait;
use dyn_clonable::clonable;
use std::{
    collections::HashMap,
    net::SocketAddr,
    time::{Duration, SystemTime},
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{Receiver, Sender},
};

pub struct Bot {
    user: User,

    config: Config,

    communications: Vec<Box<dyn Communication>>,
    event_sender: Sender<Event>,
    event_receiver: Receiver<Event>,
    action_sender: Sender<Action>,
    action_receiver: Receiver<Action>,

    custom_field: HashMap<String, String>,
}

impl Bot {
    pub fn new(event_buffer: usize, action_buffer: usize) -> Self {
        let (event_sender, event_receiver) = tokio::sync::mpsc::channel(event_buffer);
        let (action_sender, action_receiver) = tokio::sync::mpsc::channel(action_buffer);
        Self {
            user: User::default(),
            config: Config::default(),
            communications: Vec::new(),
            event_sender,
            event_receiver,
            action_sender,
            action_receiver,
            custom_field: HashMap::new(),
        }
    }

    pub fn add_communication(&mut self, communication: Box<dyn Communication>) {
        self.communications.push(communication);
    }

    async fn start(&self) {
        for communication in self.communications.iter() {
            let communication = communication.clone();
            let event_sender = self.event_sender.clone();
            tokio::spawn(async move {
                communication.start(event_sender).await;
            });
        }

        loop {}
    }

    async fn push_events(&mut self) {
        while let Some(event) = self.event_receiver.recv().await {
            for communication in self.communications.iter() {
                let event = event.clone();
                communication.push_event(event).await;
            }
        }
    }

    async fn process_actions(&self) {}
}

#[derive(Default)]
struct Config {
    message_format: MessageFormat,
    rate_limit: Duration,

    custom_field: HashMap<String, String>,
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
    async fn start(&self, event_sender: Sender<Event>);
    async fn push_event(&self, event: Event);
}

mod communications {
    use crate::*;
    pub struct HTTP {
        socket_addr: SocketAddr,
        access_token: String,
    }

    pub struct HTTPPost {
        post_url: String,
        timeout: Duration,
        secret: String,
    }

    pub struct WebSocket {
        socket_addr: SocketAddr,
        access_token: String,
    }

    impl WebSocket {
        async fn start(&self, event_sender: Sender<Event>) -> Result<(), AnyhowError> {
            let listener = TcpListener::bind(&self.socket_addr).await?;

            while let Ok((stream, addr)) = listener.accept().await {
                tokio::spawn(async move {});
            }

            Ok(())
        }
    }

    pub struct WebSocketReverse {
        connect_url: String,
        r#type: WebSocketReverseType,
        access_token: String,
    }

    pub enum WebSocketReverseType {
        API,
        Event,
        Universal,
    }
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

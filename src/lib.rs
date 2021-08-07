use actix_web::{HttpRequest, HttpResponse};
use anyhow::Error;
use async_trait::async_trait;
use dyn_clonable::clonable;
use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};
use tokio::sync::broadcast::Sender;

type Result<T> = std::result::Result<T, Error>;

pub struct OneBot {
    user: User,

    config: Config,

    event_generators: Vec<fn(event_sender: Sender<Event>) -> Result<()>>,
    actions: HashMap<String, fn(param: GeneralParams)>,
    webhook_handlers: Vec<Webhook>,

    communications: Vec<Box<dyn Communication>>,
    event_sender: Sender<Event>,

    pub custom_field: HashMap<String, String>,
}

impl OneBot {
    pub fn new(event_capacity: usize) -> Self {
        let (event_sender, _) = tokio::sync::broadcast::channel(event_capacity);

        Self {
            user: User::default(),
            config: Config::default(),
            event_generators: Vec::new(),
            actions: HashMap::new(),
            webhook_handlers: Vec::new(),
            communications: Vec::new(),
            event_sender,
            custom_field: HashMap::new(),
        }
    }

    pub fn add_communication(&mut self, communication: Box<dyn Communication>) {
        self.communications.push(communication);
    }

    pub async fn run(&mut self) {
        for event_generator in self.event_generators.iter() {
            let event_generator = event_generator.clone();
            let event_sender = self.event_sender.clone();
            tokio::spawn(async move {
                (event_generator)(event_sender).unwrap();
            });
        }

        for communication in self.communications.iter() {
            let communication = communication.clone();
            let event_sender = self.event_sender.clone();
            tokio::spawn(async move {
                communication.start(event_sender).await.unwrap();
            });
        }

        loop {}
    }

    pub fn register_event_generator(
        &mut self,
        event_generator: fn(event_sender: Sender<Event>) -> Result<()>,
    ) {
        self.event_generators.push(event_generator);
    }

    pub fn register_webhook(
        &mut self,
        path: String,
        handler: fn(req: HttpRequest) -> Result<HttpResponse>,
    ) {
        self.webhook_handlers.push(Webhook::new(path, handler));
    }

    pub fn register_action<A: Action>(&mut self, action: fn(params: GeneralParams)) {
        self.actions.insert(A::NAME, action);
    }
}

trait EmitEvent {
    fn emit(&self, event: Event) -> Result<()>;
}

impl EmitEvent for Sender<Event> {
    fn emit(&self, event: Event) -> Result<()> {
        self.send(event)?;

        Ok(())
    }
}

struct Webhook {
    path: String,
    handler: fn(req: HttpRequest) -> Result<HttpResponse>,
}

impl Webhook {
    fn new(path: String, handler: fn(req: HttpRequest) -> Result<HttpResponse>) -> Self {
        Self { path, handler }
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
    async fn start(&self, event_receiver: Sender<Event>) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct Message {
    id: i64,
    source: MessageSource,
    sender: User,

    content: MessageContent,

    custom_field: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum MessageSource {
    Private(User),
    Group(Group),
}

#[derive(Debug, Clone)]
pub enum MessageContent {
    String(String),
    Array(Vec<MessageSegment>),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Media {
    File(String),
    URL(String, bool, bool, bool), // url, cache, proxy, timeout
    Base64(String),
}

#[derive(Debug, Clone)]
pub struct Event {
    time: SystemTime,
    content: EventContent,

    custom_field: HashMap<String, String>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Notice {
    custom_field: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Request {
    custom_field: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Meta {
    custom_field: HashMap<String, String>,
}

pub trait Action {
    const NAME: String;
    type Param;
    fn parse_params(params: GeneralParams) -> Self::Param;
}

pub struct GeneralParams {}

#[derive(Debug, Clone, Default)]
pub struct User {
    id: i64,
    username: String,

    nickname: String,
    display_name: String,

    custom_field: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Group {
    id: i64,

    name: String,

    custom_field: HashMap<String, String>,
}

mod actions;
mod communications;

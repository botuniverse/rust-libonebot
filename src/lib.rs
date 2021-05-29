use std::collections::HashMap;

#[derive(Default)]
pub struct Bot {
    user: User,

    config: Config,

    custom_field: HashMap<String, String>,
}

impl Bot {
    pub fn new() -> Self {
        Bot::default()
    }
}

#[derive(Default)]
struct Config {
    communication: Vec<Communication>,
    message_format: MessageFormat,
    rate_limit: std::time::Duration,

    custom_field: HashMap<String, String>,
}

enum MessageFormat {
    String,
    Array,
}

impl Default for MessageFormat {
    fn default() -> Self {
        MessageFormat::String
    }
}

enum Communication {
    HTTP(HTTP),
    HTTPPost(HTTPPost),
    WebSocket(WebSocket),
    WebSocketReverse(WebSocketReverse),
}

pub struct HTTP {
    bind_ip: std::net::IpAddr,
    port: u16,
    access_token: String,
}

pub struct HTTPPost {
    post_url: String,
    timeout: std::time::Duration,
    secret: String,
}

pub struct WebSocket {
    bind_ip: std::net::IpAddr,
    port: u16,
    access_token: String,
}

pub struct WebSocketReverse {
    connect_url: String,
    r#type: WebSocketReverseType,
    access_token: String,
}

enum WebSocketReverseType {
    API,
    Event,
    Universal,
}

pub struct Message {
    id: i64,
    source: MessageSource,
    sender: User,

    content: MessageContent,

    custom_field: HashMap<String, String>,
}

pub enum MessageSource {
    Private(User),
    Group(Group),
}

pub enum MessageContent {
    String(String),
    Array(Vec<MessageSegment>),
}

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

pub enum Media {
    File(String),
    URL(String, bool, bool, bool), // url, cache, proxy, timeout
    Base64(String),
}

pub struct Event {
    time: std::time::SystemTime,
    content: EventContent,

    custom_field: HashMap<String, String>,
}

pub enum EventContent {
    Message(Message),
    Notice(Notice),
    Request(Request),
    Meta(Meta),
}

pub struct Notice {
    custom_field: HashMap<String, String>,
}

pub struct Request {
    custom_field: HashMap<String, String>,
}

pub struct Meta {
    custom_field: HashMap<String, String>,
}

#[derive(Default)]
pub struct User {
    id: i64,
    username: String,

    nickname: String,
    display_name: String,

    sex: String,

    custom_field: HashMap<String, String>,
}

pub struct Group {
    id: i64,

    name: String,

    custom_field: HashMap<String, String>,
}

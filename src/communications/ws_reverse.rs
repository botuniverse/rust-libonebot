#[derive(Clone)]
pub struct WebSocketReverse {
    connect_url: String,
    r#type: WebSocketReverseType,
    access_token: String,
}

#[derive(Clone)]
pub enum WebSocketReverseType {
    API,
    Event,
    Universal,
}

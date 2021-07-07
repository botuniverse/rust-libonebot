use std::time::Duration;

#[derive(Clone)]
pub struct HTTPPost {
    post_url: String,
    timeout: Duration,
    secret: String,
}

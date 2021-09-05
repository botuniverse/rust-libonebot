use std::time::Duration;

#[derive(Clone)]
pub struct HTTPWebHook {
    post_url: String,
    timeout: Duration,
    secret: String,
}

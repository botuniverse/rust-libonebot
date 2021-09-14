#[derive(Clone)]
pub struct HTTPWebHook {
    post_url: String,
    secret: String,
}

use crate::Action;
use std::collections::HashMap;

pub struct SendMessage {}

pub struct SendMessageParams {}

impl Action for SendMessageParams {
    const NAME: &'static str = "send_message";
    type Params = SendMessageParams;
    fn parse_params(p: HashMap<String, String>) -> Self::Params {
        Self::Params {}
    }
}

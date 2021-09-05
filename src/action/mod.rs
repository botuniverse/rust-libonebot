use std::collections::HashMap;

pub trait Action {
    const NAME: &'static str;
    type Params;
    fn parse_params(params: HashMap<&str, String>) -> Self::Params;
}

pub struct SendMessage {}

pub struct SendMessageParams {}

impl Action for SendMessageParams {
    const NAME: &'static str = "send_message";
    type Params = SendMessageParams;
    fn parse_params(p: HashMap<&str, String>) -> Self::Params {
        Self::Params {}
    }
}

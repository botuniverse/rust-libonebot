// 库代码
use std::collections::BTreeMap;

pub type ApiResult = i32;

struct GeneralParams;

impl GeneralParams {
    fn get_int(&self) -> i32 {
        233
    }
    fn get_string(&self) -> String {
        String::new()
    }
}

trait Action {
    const NAME: &'static str;
    type Param;
    fn parse_params(p: GeneralParams) -> Self::Param;
}

pub struct SendMessage;
pub struct SendMessageParams(pub i32);
impl Action for SendMessage {
    const NAME: &'static str = "SendMessage";
    type Param = SendMessageParams;
    fn parse_params(p: GeneralParams) -> SendMessageParams {
        SendMessageParams(p.get_int())
    }
}

pub struct SendLike;
pub struct SendLikeParams(pub String);
impl Action for SendLike {
    const NAME: &'static str = "SendMessage";
    type Param = SendLikeParams;
    fn parse_params(p: GeneralParams) -> SendLikeParams {
        SendLikeParams(p.get_string())
    }
}

pub struct Manager<'a> {
    actions: BTreeMap<&'static str, Box<dyn FnMut(GeneralParams) -> ApiResult + 'a>>,
}

impl<'a> Manager<'a> {
    fn new() -> Self {
        Manager {
            actions: Default::default(),
        }
    }
    fn register_action<A: Action, C: 'a + FnMut(A::Param) -> ApiResult>(
        &mut self,
        mut on_action: C,
    ) {
        self.actions
            .insert(A::NAME, Box::new(move |p| on_action(A::parse_params(p))));
    }
}

// 用户代码
fn main() {
    let mut manager = Manager::new();
    manager.register_action::<SendMessage, _>(|s| {
        eprintln!("{}", s.0);
        0
    });
    manager.register_action::<SendLike, _>(|s| {
        eprintln!("{}", s.0);
        1
    });
}

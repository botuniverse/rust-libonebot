// Plan A: register handlers to libonebot module

use libonebot::*;

// 类似 telegram 这样的有 long polling 机制的平台，可以注册 event generator 来获取事件
async fn event_generator(onebot: &OneBot) -> Result<()> {
    loop {
        let updates = telegram::get_updates(); // 调用平台长轮询 api 获取事件
        for update in updates {
            // 对于 OneBot 核心事件集中的事件，需要转换成对应的类型
            let event = convert_event(update);

            // 从一个 channel 发送事件，libonebot 的某个 task 会在另一端拿到，进行上报给用户的逻辑
            onebot.emit_event(event);

            // 对于不在核心事件集中的事件，需要考虑一下是用单独的 emit_extended_event
            // 还是直接让用户实现一个 Event: Serialize + Deserialize
            // 因为我们需要强制这些事件的 detail_type 包含一个平台前缀，比如 `qq_blahblah`
        }
    }
}

// 同时也允许用户注册更原始的 http handler 来应对聊天平台只支持 webhook 上报事情的情况
async fn webhook_handler(req: HttpRequest) -> Result<HttpResponse> {
    // 例如在微信公众号平台，需要用 webhook 接收 xml 数据，无法复用 onebot http api 的逻辑
}

async fn send_message(params: SendMessageParams /* 从 web 请求反序列化而来 */)
    -> Result<SendMessageResult /* 返回结果会序列化为 web 请求的响应 */> {
    let message = blahblah(params.message);
    let chat_id = params.group_id.clone();
    let res = telegram::send_xxx_message(chat_id, message);
    Ok(convert_result(res))
}

async fn some_extended_action(params: GenericParams) -> Result<GenericResult> {
    // 用户自行从 params 里面拿参数
    let user_id = params.get_int("user_id");
}

async fn main() {
    let ob = OneBot::new();

    // 这会在 onebot 启动时 spawn 一个 task，来跑用户提供的函数
    // 若 task 不正常返回了（返回 Err），则可以自动重启该 task
    ob.register_event_generator(event_generator);

    // 对于只支持 webhook 的平台，可以往一个 path 上注册 handler
    // 不需要提供很大自由度，避免跟 onebot 的 api 冲突，下面的代码会
    // 把 handler 注册到 /webhook/wxmp
    ob.register_webhook("wxmp", webhook_handler);

    // 对 OneBot 核心动作集中的动作，可以用类型安全的方式保证用户获得正确的参数，返回正确的结果
    ob.register_action::<SendMessage>(send_message);

    // 实现自定义的扩展动作
    ob.register_extended_action("qq" /* prefix */, "some_extended_action" /* action name */,
                                some_extended_action);

    ob.run().await;
}
# (WIP) Rust LibOneBot `libonebot`

[![onebot-badge][]][onebot] [![ci-badge][]][ci] [![mit-badge][]][mit]

> 目前 API 仍在重构中，尚不稳定，随时可能发生不兼容的变化

Rust LibOneBot 可以帮助 OneBot 实现者快速在新的聊天机器人平台实现 OneBot v12 接口标准。

具体而言，Rust LibOneBot 通过 `OneBot`、`Config`、`Action`、`Event` 等类型的抽象，让 OneBot 实现者只需编写少量代码即可完成一个 OneBot 实现，而无需关心过多 OneBot 标准所定义的通信方式的细节。

基于 LibOneBot 实现 OneBot 时，OneBot 实现者只需专注于编写与聊天机器人平台对接的逻辑，包括通过长轮询或 webhook 方式从机器人平台获得事件，并将其转换为 OneBot 事件，以及处理 OneBot 动作请求，并将其转换为对机器人平台 API 的调用。

## 用法

一个 OneBot echo 实现：

```rust
use libonebot::{config::DefaultConfigFile, Event, Message, OneBot, Result, User};

#[tokio::main]
async fn main() -> Result<()> {
    let onebot = OneBot::new("nothing"); // 创建 OneBot 实例
    onebot.set_default_config(); // 创建默认 Config
    onebot.register_action_handler("echo", |params| {
        // 当收到的 json 为 {"action" : "echo", "params" : {"message" : a_string }} 时，返回“received: a_string”，否则返回空
        if let serde_json::Value::Object(params) = params {
            if let Some(message) = params.get("message") {
                if let serde_json::Value::String(s) = message {
                    println!("received: {}", s);
                    return format!("received: {}", s);
                }
            }
        }
        format!("")
    }

    onebot.run().await?; // 运行 OneBot 实例

    Ok(())
}
```

通过交互命令行输入“私聊消息”的实现：[待实现]()。

关于上面示例中所涉及的类型、函数的更多细节，请[等待 Rust doc 的发布]()。

## 致谢

- 感谢 @richardchien 提出了 OneBot 协议思路
- 感谢 @spacemeowx2 和 @bdbai 为本项目实现提供了一些建议

[onebot-badge]: https://img.shields.io/badge/OneBot-v12-black?logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAHAAAABwCAMAAADxPgR5AAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAAxQTFRF////29vbr6+vAAAAk1hCcwAAAAR0Uk5T////AEAqqfQAAAKcSURBVHja7NrbctswDATQXfD//zlpO7FlmwAWIOnOtNaTM5JwDMa8E+PNFz7g3waJ24fviyDPgfhz8fHP39cBcBL9KoJbQUxjA2iYqHL3FAnvzhL4GtVNUcoSZe6eSHizBcK5LL7dBr2AUZlev1ARRHCljzRALIEog6H3U6bCIyqIZdAT0eBuJYaGiJaHSjmkYIZd+qSGWAQnIaz2OArVnX6vrItQvbhZJtVGB5qX9wKqCMkb9W7aexfCO/rwQRBzsDIsYx4AOz0nhAtWu7bqkEQBO0Pr+Ftjt5fFCUEbm0Sbgdu8WSgJ5NgH2iu46R/o1UcBXJsFusWF/QUaz3RwJMEgngfaGGdSxJkE/Yg4lOBryBiMwvAhZrVMUUvwqU7F05b5WLaUIN4M4hRocQQRnEedgsn7TZB3UCpRrIJwQfqvGwsg18EnI2uSVNC8t+0QmMXogvbPg/xk+Mnw/6kW/rraUlvqgmFreAA09xW5t0AFlHrQZ3CsgvZm0FbHNKyBmheBKIF2cCA8A600aHPmFtRB1XvMsJAiza7LpPog0UJwccKdzw8rdf8MyN2ePYF896LC5hTzdZqxb6VNXInaupARLDNBWgI8spq4T0Qb5H4vWfPmHo8OyB1ito+AysNNz0oglj1U955sjUN9d41LnrX2D/u7eRwxyOaOpfyevCWbTgDEoilsOnu7zsKhjRCsnD/QzhdkYLBLXjiK4f3UWmcx2M7PO21CKVTH84638NTplt6JIQH0ZwCNuiWAfvuLhdrcOYPVO9eW3A67l7hZtgaY9GZo9AFc6cryjoeFBIWeU+npnk/nLE0OxCHL1eQsc1IciehjpJv5mqCsjeopaH6r15/MrxNnVhu7tmcslay2gO2Z1QfcfX0JMACG41/u0RrI9QAAAABJRU5ErkJggg==
[onebot]: https://1bot.dev
[ci-badge]: https://img.shields.io/github/workflow/status/botuniverse/rust-libonebot/ci?style=flat
[ci]: https://github.com/botuniverse/libonebot/actions
[mit-badge]: https://img.shields.io/github/license/botuniverse/rust-libonebot?style=flat
[mit]: https://github.com/botuniverse/rust-libonebot/blob/master/LICENSE

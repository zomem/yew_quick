
## yew 框架 快速开发组件库

目前组件如下：
|  组件              | 说明  |
|  ----             | ----  |
| Block             | 块（Fragment） |
| Box               | 同div |
| Flex              | 同flex布局的div |
| Image             | 图片 |
| Line              | 线   |
| Text              | 文本 |
| TextEllipsis      | 文本，带省略行数 |


## 使用如下：
```rust
use yew::prelude::*;
use yew_quick::{Box, Flex, FlexWay, Text, Image};

#[function_component]
fn App() -> Html {
    // size 表示 长 宽
    html! {
        <Box size="100% 500">
            <Flex flex={FlexWay::Frbc} size="200 600">
                <Image size="50" src="" />
                <Text color="red" >{"文本"}</Text>
            </Flex>
        </Box>
    }
}
```
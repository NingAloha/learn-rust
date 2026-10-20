### 绑定到值的模式

`match` 分支的另一个有用特性是它们可以绑定到与模式匹配的值的部分。这是我们如何从 `enum` 变体中提取值。

例如，我们可以将一个 `enum` 变体更改为在其中存储数据。从1999年到2008年，美国铸造了带有50个州不同设计的25美分硬币。在其他硬币上没有州设计，所以只有25美分硬币有这个额外的值。我们可以通过更改 `Quarter` 变体以包含一个存储在内部的 `UsState` 值来添加此信息，这样我们就在下面的代码片段中完成了这一操作。

```rust
#[derive(Debug)] // 这样我们可以马上检查状态
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

#### 一个 `Coin` 枚举，其中 `Quarter` 变体还包含一个 `UsState` 值

假设我们的一个朋友正在尝试收集所有50个州的25美分硬币。当我们按硬币类型整理散币时，我们也会喊出每个25美分硬币的相关州的名称，以便如果那是我们的朋友还没有的州，他们可以将其添加到他们的收藏中。

在这段代码的 `match` 表达式中，我们将一个名为 `state` 的变量添加到匹配变体 `Coin::Quarter` 的值的模式中。当 `Coin::Quarter` 匹配时，`state` 变量将绑定到该25美分硬币的州值。然后我们可以在该分支的代码中使用 `state`，如下所示：

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("来自 {:?} 的州25美分硬币!", state);
            25
        }
    }
}
```

如果我们调用 `value_in_cents(Coin::Quarter(UsState::Alaska))`，`coin` 将是 `Coin::Quarter(UsState::Alaska)`。当我们将该值与每个 `match` 分支进行比较时，直到达到 `Coin::Quarter(state)`，才有匹配项。在这一点上，对 `state` 的绑定将是值 `UsState::Alaska`。然后我们可以在 `println!` 表达式中使用该绑定，从而从 `Coin` 枚举中变体 `Quarter` 中获取内部的州值。
### 带数据和方法的枚举

让我们来看下面代码中的一个枚举示例：这一个在其变体中嵌入了各种类型。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

#### 一个 `Message` 枚举，其变体存储不同数量和类型的值

这个枚举有四个类型不同的变体：

* `Quit` 完全没有关联的数据。
* `Move` 包含了一个匿名结构体。
* `Write` 包含一个 `String`。
* `ChangeColor` 包含三个 `i32` 值。

定义像上面列表中这样的变体枚举，类似于定义不同种类的结构体定义，只是枚举不使用 `struct` 关键字，所有变体都归在 `Message` 类型下。以下结构体可以容纳之前枚举变体所持有的相同数据：

```rust
struct QuitMessage; // 单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体
```

但是如果我们使用不同的结构体，每个都有自己的类型，我们无法像前面代码片段中定义的 `Message` 枚举那样轻松地定义一个函数接受这些类型的消息，因为 `Message` 是一个单一类型。

枚举和结构体还有一个相似之处：就像我们能够使用 `impl` 为结构体定义方法一样，我们也能为枚举定义方法。以下是一个名为 `call` 的方法，我们可以在我们的 `Message` 枚举上定义：

```rust
impl Message {
    fn call(&self) {
        // 方法体在这里定义
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

方法体将使用 `self` 来获取我们调用方法的值。在此示例中，我们创建了一个变量 `m` 其值为 `Message::Write(String::from("hello"))`，这就是在 `m.call()` 运行时在 `call` 方法体中 `self` 所代表的内容。

让我们看看标准库中另一个非常常见且有用的枚举：`Option`。
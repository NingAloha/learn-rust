### 调用没有默认实现的方法

即使其他方法没有默认实现，默认实现也可以调用同一个 trait 中的其他方法。通过这种方式，trait 可以提供很多有用的功能，而只要求实现者指定其中一小部分。例如，我们可以定义一个 `Summary` trait，其中 `summarize_author` 方法的实现是必需的，然后定义一个 `summarize` 方法，该方法具有默认实现并调用 `summarize_author` 方法：

```rust,noplayground
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(阅读全文来自 {}...)", self.summarize_author())
    }
}
```

要使用此版本的 `Summary`，我们只需要在我们为某个类型实现此 trait 时定义 `summarize_author`：

```rust,ignore
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

在定义了 `summarize_author` 之后，我们可以在 `Tweet` 结构体的实例上调用 `summarize`，而 `summarize` 的默认实现将调用我们提供的 `summarize_author` 定义。因为我们已经实现了 `summarize_author`，所以 `Summary` trait 为我们提供了 `summarize` 方法的行为，而不需要我们编写更多的代码。

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "当然，如你可能已经知道的，人们",
    ),
    reply: false,
    retweet: false,
};

println!("1 条新推文: {}", tweet.summarize());
```

此代码打印 `1 条新推文: (阅读全文来自 @horse_ebooks...)`。

请注意，不可能从一个覆盖实现中调用同一方法的默认实现。
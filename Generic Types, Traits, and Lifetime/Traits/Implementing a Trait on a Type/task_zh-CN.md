### 在类型上实现一个 Trait

现在我们已经使用 `Summary` trait 定义了所需的行为，我们可以在媒体聚合器的类型上实现它。下面的代码展示了如何在 `NewsArticle` 结构体上实现 `Summary` trait，使用标题、作者和位置来创建 `summarize` 的返回值。对于 `Tweet` 结构体，我们将 `summarize` 定义为用户名加上整条推文的文本，假设推文的内容已经限制在 280 个字符以内。

```rust,noplayground
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

#### 在 `NewsArticle` 和 `Tweet` 类型上实现 `Summary` trait

在类型上实现一个 trait 类似于实现常规方法。区别在于，在 `impl` 之后，我们放置我们想要实现的 trait 名称，然后使用 `for` 关键字，然后指定我们要实现 trait 的类型名称。在 `impl` 块内，我们放置 trait 定义中定义的方法签名。我们不用在每个签名后添加分号，而是用花括号来填充方法体，以此为特定类型的方法赋予我们期望的特定行为。

在实现了 trait 之后，我们可以在 `NewsArticle` 和 `Tweet` 的实例上调用这些方法，就像我们调用常规方法一样，如下所示：

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

这段代码会打印 `1 new tweet: horse_ebooks: of course, as you probably already know, people`。

注意，因为我们在"在 `NewsArticle` 和 `Tweet` 类型上实现 `Summary` trait"这段代码中，将 `Summary` trait 和 `NewsArticle` 和 `Tweet` 类型定义在同一个 *lib.rs* 文件里，所以它们都在同一个作用域中。假设这个 *lib.rs* 是我们称为 `aggregator` 的 crate 中的，而其他人希望在他们的库作用域内定义的结构体上使用我们 crate 的功能来实现 `Summary` trait，他们首先需要将 trait 引入到他们的作用域中。他们可以通过指定 `use aggregator::Summary;` 来实现，这样他们就可以为他们的类型实现 `Summary`。其他 crate 要实现 `Summary` trait，还需要它是一个公共 trait，因为我们在 Listing 10-12 中在 `trait` 前放置了 `pub` 关键字，所以它是公共的。

trait 实现需要注意的一点限制是，我们只有在 trait 或类型是我们 crate 本地的情况下，才能在某个类型上实现一个 trait。例如，我们可以在自定义类型 `Tweet` 上实现标准库中的 `Display` trait，作为 `aggregator` crate 功能的一部分，因为 `Tweet` 类型是 `aggregator` crate 本地的。我们也可以在 `aggregator` crate 中实现 `Vec<T>` 上的 `Summary`，因为 `Summary` trait 是我们 `aggregator` crate 本地的。

但我们不能在外部类型上实现外部的 trait。例如，我们不能在 `aggregator` crate 中实现 `Vec<T>` 上的 `Display` trait，因为 `Display` 和 `Vec<T>` 都是在标准库中定义的，并且不是 `aggregator` crate 本地的。这项限制是程序属性的一部分，称为*一致性*，更具体地说是*孤儿规则*，因为父类型不在场。这个规则确保了别人不会破坏你的代码，反之亦然。如果没有这项规则，两个 crate 可以为同一种类型实现同一 trait，Rust 就不知道该使用哪个实现。
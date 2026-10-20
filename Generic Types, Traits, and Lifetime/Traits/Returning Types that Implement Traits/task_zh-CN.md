### 返回实现特征的类型

我们还可以在返回位置使用 `impl Trait` 语法，以返回某个类型实现特征的值，如下所示：

```rust,ignore
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

通过使用 `impl Summary` 作为返回类型，我们指定 `returns_summarizable` 函数返回某种实现了 `Summary` 特征的类型，而不必命名具体类型。在这种情况下，`returns_summarizable` 返回一个 `Tweet`，但调用此函数的代码不会知道这一点。

仅通过实现的特征来指定返回类型的能力在闭包和迭代器的上下文中特别有用，我们将在第十三章中讨论。闭包和迭代器创建的类型只有编译器知道，或者类型很长而难以指定。`impl Trait` 语法允许您简洁地指定一个函数返回某个实现了 `Iterator` 特征的类型，而不必写出一个很长的类型。

然而，您只能在返回单一类型时使用 `impl Trait`。例如，这段代码返回 `NewsArticle` 或 `Tweet`，并将返回类型指定为 `impl Summary` 时将无法工作：

```rust,ignore,does_not_compile
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

由于编译器中 `impl Trait` 语法的实现限制，返回 `NewsArticle` 或 `Tweet` 中的一个是不允许的。在《Rust编程语言》第十七章 [“使用允许不同类型值的特征对象”][using-trait-objects-that-allow-for-values-of-different-types] 部分，我们将介绍如何编写具有此类行为的函数。

[using-trait-objects-that-allow-for-values-of-different-types]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
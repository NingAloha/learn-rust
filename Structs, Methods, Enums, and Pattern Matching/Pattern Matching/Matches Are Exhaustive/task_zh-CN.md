### match 是穷尽性的

关于 `match` 还有一个方面我们需要讨论。看看这个 `plus_one` 函数的版本，它有一个漏洞而且无法编译：

```rust,ignore,does_not_compile
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

我们没有处理 `None` 情况，所以这段代码会导致错误。幸运的是，这是 Rust 知道如何捕捉的错误。如果尝试编译这段代码，我们会得到以下错误：

```console
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:3:15
    |
3   |         match x {
    |               ^ pattern `None` not covered
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `Option<i32>`
```

Rust 知道我们没有覆盖所有可能的情况，甚至知道我们忘记了哪个模式！在 Rust 中，匹配是*穷尽性的*：我们必须穷尽所有可能性才能使代码有效。特别是在 `Option<T>` 的情况下，当 Rust 阻止我们忘记显式处理 `None` 情况时，它保护我们避免在可能为 null 时假定我们有一个值，从而使得前面讨论的价值数十亿的错误成为不可能。
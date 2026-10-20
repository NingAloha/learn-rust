### 类函数宏

类函数宏定义的宏看起来像函数调用。与 `macro_rules!` 宏类似，它们比函数更灵活；例如，它们可以接受不确定数量的参数。然而，`macro_rules!` 宏只能使用我们在[“使用 `macro_rules!` 进行通用元编程的声明式宏”](https://doc.rust-lang.org/stable/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming)章节中讨论的匹配样式语法来定义。类函数宏接收一个 `TokenStream` 参数，并且它们的定义使用 Rust 代码来操作该 `TokenStream`，就像其他两种类型的过程宏一样。一个类函数宏的例子是 `sql!` 宏，它可能被这样调用：

```rust
    let sql = sql!(SELECT * FROM posts WHERE id=1);
```

这个宏会解析其中的 SQL 语句并检查它的语法是否正确，这比 `macro_rules!` 宏要复杂得多。`sql!` 宏将会这样定义：

```rust
    #[proc_macro]
    pub fn sql(input: TokenStream) -> TokenStream {
```

这种定义类似于自定义派生宏的签名：我们接收括号内的标记，并返回我们希望生成的代码。
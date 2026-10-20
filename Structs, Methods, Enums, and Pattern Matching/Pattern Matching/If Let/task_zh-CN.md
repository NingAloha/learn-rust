## 使用 `if let` 简化控制流

`if let` 语法让你可以将 `if` 和 `let` 结合在一起，以更简洁的方式处理匹配一种模式的值，同时忽略其他值。请看下面的程序，它匹配一个 `Option<u8>` 值，仅在值为 3 时执行代码。

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

我们想对 `Some(3)` 匹配做些事情，但对其他任何 `Some<u8>` 值或 `None` 值则不做处理。为了满足 `match` 表达式，我们必须在处理完一个变体后添加 `_ => ()`，这会增加很多样板代码。

相反，我们可以使用 `if let` 来以更简洁的方式编写。下面的代码与前面代码片段中的 `match` 具有相同的行为：

```rust
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}
```

`if let` 语法使用一个等号分隔的模式和表达式。它的工作方式与 `match` 相同，在 `match` 中表达式被提供给 `match`，而模式是它的第一个分支。

使用 `if let` 意味着减少输入、降低缩进和减少样板代码。然而，你会失去 `match` 强制的穷尽检查。在选择 `match` 和 `if let` 之间时，取决于在特定情况下你所做的事情，以及减少代码量是否适合于失去穷尽检查的权衡。

换句话说，你可以将 `if let` 视为 `match` 的语法糖，在值匹配一种模式时运行代码，然后忽略所有其他值。

我们可以在 `if let` 中包含一个 `else`。与 `else` 一起的代码块与 `match` 表达式中相当于 `if let` 和 `else` 的 `_` 案例中的代码块相同。回想前一节中的 `Coin` 枚举定义，其中 `Quarter` 变体还包含一个 `UsState` 值。如果我们想在看到非季度的硬币时计数，同时宣布每个季度的州名，可以这样使用 `match` 表达式：

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

或者我们可以这样使用 `if let` 和 `else` 表达式：

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

如果在某种情况下你的程序逻辑使用 `match` 表示过于冗长，记住在 Rust 工具箱中还有 `if let`。
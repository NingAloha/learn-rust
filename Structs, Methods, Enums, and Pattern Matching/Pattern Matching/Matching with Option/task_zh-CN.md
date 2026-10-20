### 使用 `Option<T>` 进行匹配

在上一节中，我们想要从使用 `Option<T>` 的 `Some` 情况中获取内部的 `T` 值；我们也可以像处理 `Coin` 枚举那样使用 `match` 来处理 `Option<T>`！我们将比较 `Option<T>` 的变体，而不是比较硬币，但 `match` 表达式的工作方式保持不变。

假设我们想编写一个函数，该函数接收一个 `Option<i32>`，如果内部有值，则将该值加 1。如果内部没有值，函数应返回 `None` 并且不尝试执行任何操作。

感谢 `match`，这个函数很容易编写，看起来像下面的代码。

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

#### 使用 `match` 表达式的 `Option<i32>` 的函数

让我们更详细地检查第一次执行 `plus_one`。当我们调用 `plus_one(five)` 时，`plus_one` 函数体内的变量 `x` 将具有值 `Some(5)`。然后我们将其与每个匹配臂进行比较。

```rust,ignore
None => None,
```

`Some(5)` 值不匹配模式 `None`，因此我们继续到下一个臂。

```rust,ignore
Some(i) => Some(i + 1),
```

`Some(5)` 是否匹配 `Some(i)`？当然匹配！我们有相同的变体。`i` 绑定到 `Some` 中包含的值，因此 `i` 拿到值 `5`。然后执行匹配臂中的代码，因此我们将 `i` 的值加 1，并创建一个包含我们和总和 `6` 的新的 `Some` 值。

现在让我们考虑示例 6-5 中第二次调用 `plus_one`，其中 `x` 是 `None`。我们进入 `match` 并与第一个臂比较。

```rust,ignore
None => None,
```

匹配了！没有要添加的值，所以程序停止并返回 `=>` 右侧的 `None` 值。由于第一个臂匹配，所以不再比较其他臂。

在许多情况下，结合 `match` 和枚举是很有用的。在 Rust 代码中您会经常看到这个模式：对枚举进行 `match`，将变量绑定到内部的数据，然后根据它执行代码。起初这有点棘手，但一旦习惯了，您会希望在所有语言中都有它。这是一种用户们一致喜欢的特性。
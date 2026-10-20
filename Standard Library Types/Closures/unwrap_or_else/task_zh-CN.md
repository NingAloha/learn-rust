#### 示例：`unwrap_or_else`

让我们看看 `Option<T>` 上的 `unwrap_or_else` 方法的定义，我们在[之前](course://Standard Library Types/Closures/Capturing the Environment with Closures)使用过：

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

回顾一下，`T` 是泛型类型，代表 `Option` 的 `Some` 变体中的值类型。类型 `T` 也是 `unwrap_or_else` 函数的返回类型：比如，调用 `unwrap_or_else` 的代码对于 `Option<String>` 会得到一个 `String`。

接下来，注意到 `unwrap_or_else` 函数有一个额外的泛型类型参数 `F`。类型 `F` 是参数 `f` 的类型，即我们在调用 `unwrap_or_else` 时提供的闭包。

在泛型类型 `F` 上指定的特征约束是 `FnOnce() -> T`，这意味着 `F` 必须能够至少被调用一次，不接受任何参数，并返回一个 `T`。在特征约束中使用 `FnOnce` 表达了 `unwrap_or_else` 最多只会调用 `f` 一次的限制。在 `unwrap_or_else` 的主体中，我们可以看到如果 `Option` 是 `Some`，`f` 不会被调用。如果 `Option` 是 `None`，`f` 会被调用一次。因为所有的闭包都实现了 `FnOnce`，`unwrap_or_else` 可以接受最多种类的闭包，并且尽可能灵活。

> 注意：函数也可以实现全部三个 `Fn` 特征。如果我们要做的事情不需要从环境中捕获一个值，我们可以使用一个函数名而非闭包来实现 `Fn` 特征。例如，对于一个 `Option<Vec<T>>` 值，我们可以调用 `unwrap_or_else(Vec::new)` 来获得一个新的空向量，如果值为 `None`。
### 使用 `as` 关键字提供新名称

关于在同一作用域使用 `use` 导入两个同名类型的问题，还有另一种解决方案：在路径之后，我们可以使用 `as` 和新的本地名称，或者说别名，为类型指定一个新名称。下一个例子展示了通过重命名其中一个 `Result` 类型使用 `as` 导入代码的另一种写法。

```rust
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
    }

    fn function2() -> IoResult<()> {
    }
```

##### 使用 as 关键字在导入时重命名类型

在第二个 `use` 语句中，我们选择了 `IoResult` 作为 `std::io::Result` 类型的新名称，这样它就不会与我们也引入作用域的 `std::fmt` 的 `Result` 冲突。以上两个例子都被认为是惯用方式，选择权在你手中！
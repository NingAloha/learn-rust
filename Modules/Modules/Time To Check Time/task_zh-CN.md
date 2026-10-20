## 是时候检查时间了

您可以使用 `use` 关键字从任何地方引入模块路径，特别是从 Rust 标准库引入到您的作用域中。 引入 `std::time` 模块中的 `SystemTime` 和 `UNIX_EPOCH`。如果可以用一行代码做到这一点，还有额外的风格加分！


让代码编译成功！

<div class="hint">

`UNIX_EPOCH` 和 `SystemTime` 是在 `std::time` 模块中声明的。添加一个 `use` 语句，将这两个引入到作用域中。您可以使用嵌套路径或 `glob` 操作符，仅用一行代码将它们引入。

</div>
### 通配符操作符

如果我们想要将路径中定义的_所有_公共项引入作用域，可以指定该路径，后面加上通配符`*`：

```rust
    use std::collections::*;
```

这个`use`语句将`std::collections`中定义的所有公共项引入当前作用域。使用通配符时要小心！通配符可能使得判断哪些名称在作用域中以及程序中某个名称定义的位置变得更加困难。

通配符通常在测试时使用，用于将所有测试对象引入`tests`模块；我们将在第11章的[“如何编写测试”](https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html#how-to-write-tests)部分中讨论这一点。通配符有时也作为标准库预导入模式的一部分使用：关于该模式的更多信息，请参见[标准库文档](https://doc.rust-lang.org/std/prelude/index.html#other-preludes)。

_您可以参考Rust编程语言书籍的以下章节：[使用use关键字引入路径](https://doc.rust-lang.org/stable/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#bringing-paths-into-scope-with-the-use-keyword)_
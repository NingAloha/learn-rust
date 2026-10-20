## 总结

Rust的错误处理功能旨在帮助您编写更强健的代码。`panic!`宏表示您的程序处于无法处理的状态，并允许您告诉进程停止，而不是尝试继续使用无效或错误的值。`Result`枚举使用Rust的类型系统来表明操作可能会以一种您的代码可以恢复的方式失败。您可以使用`Result`让调用您的代码的代码知道它需要处理潜在的成功或失败。在适当的情况下使用`panic!`和`Result`将使您的代码在面对不可避免的问题时更加可靠。

请参阅[The Book](https://doc.rust-lang.org/stable/book/ch09-03-to-panic-or-not-to-panic.html)中关于在您自己的代码中使用`panic`或`Result`的想法和指南的部分。
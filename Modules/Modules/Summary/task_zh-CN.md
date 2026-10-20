## 概述

Rust允许您将一个包拆分为多个crate，并将一个crate拆分为多个模块，以便您可以从一个模块引用在另一个模块中定义的项目。您可以通过指定绝对路径或相对路径来实现这一点。这些路径可以通过`use`语句引入作用域，从而在该作用域中对该项目的多次使用使用更短的路径。模块代码默认是私有的，但您可以通过添加`pub`关键字来使定义公开。

_您可以参考Rust编程语言书中的以下章节：[将模块分离到不同的文件中](https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html#separating-modules-into-different-files)_
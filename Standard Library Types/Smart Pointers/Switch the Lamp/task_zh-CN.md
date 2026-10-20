## 切换灯

在本任务中，我们需要一个灯和若干个外部开关，这些开关能够像以下场景一样操作灯：

* 创建一个灯
* 为灯创建一个开关
* 为同一个灯创建另一个开关
* 操作不同的开关并确保灯打开，然后再次关闭

很遗憾，Rust 不允许我们对灯有多个可变引用。如果你在 `main.rs` 尝试这样做，会立即看到这一点。

您的目标是编辑 `lamp` 和 `switchers` 模块，以便 `main` 可以编译并运行。
一种方法是在 `Lamp` 中引入*内部可变性*，通过 [`std::cell::Cell` 类型](https://doc.rust-lang.org/std/cell/struct.Cell.html)。

你不应该修改 `main.rs` 中的代码，它应该按现状工作。

<div class="hint">
一路上不断删除 <code>mut</code> 修饰符。对 Rust 来说，一切都应是不可变的，以便我们可以对同一个值有多个引用。
</div>

<div class="hint">查阅 <a href="https://doc.rust-lang.org/std/cell/struct.Cell.html">文档</a>以了解如何在 <code>Cell</code> 中访问和修改一个值。</div>
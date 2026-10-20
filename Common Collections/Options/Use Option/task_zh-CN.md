## 想来点冰淇淋吗？

我们有一个 `maybe_ice_cream` 函数，用来返回冰箱里剩下多少冰淇淋。
如果是在晚上10点之前，还剩下5块。到了晚上10点，有人会把它们全吃光，所以就没有了 :(

你的任务是完成 `maybe_ice_cream` 函数的实现，并修复其后的测试。

要了解 Option<T>，请查看以下链接：

- [Option 枚举格式](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-enum-definitions)
- [Option 模块文档](https://doc.rust-lang.org/std/option/)
- [Option 枚举文档](https://doc.rust-lang.org/std/option/enum.Option.html)


<div class="hint">选项可以具有一个<code>Some</code>值，与内部值，或者一个<code>None</code>值，没有内部值。
获取内部值有多种方法，你可以使用<code>unwrap</code>，或模式匹配。直接解包是最简单的，但怎样才能安全地解包，以避免后续出现崩溃呢？</div>
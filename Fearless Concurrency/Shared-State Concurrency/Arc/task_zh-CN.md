## 使用 `Arc`

通过为 `shared_numbers` 填写一个值并在某处为 `child_numbers` 创建初始绑定，使这段代码能够编译。尽量不要创建 `numbers` Vec 的任何副本！

请查看 Rust 书籍中关于[共享状态并发](https://doc.rust-lang.org/book/2018-edition/ch16-03-shared-state.html)的章节。

<div class="hint">

  使 `shared_numbers` 成为从 numbers 向量转换而来的 `Arc`。
  然后，为了避免创建 numbers 的副本，您需要在循环内但仍在主线程中创建 `child_numbers`。

  `child_numbers` 应该是 numbers 的 Arc 的副本，而不是 numbers 的线程本地副本。
</div>

<div class="hint">如果您理解了基础概念，这是一个简单的练习，但如果这对您来说太困难，建议首先阅读（或完成）"Fearless Concurrency" 部分。或者，通读书中<a href="https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html">第16章</a>。
</div>
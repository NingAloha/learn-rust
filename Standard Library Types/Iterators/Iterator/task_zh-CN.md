## 处理除法

这比大多数其他练习更复杂！ 你能做到！

这是你的任务，如果你选择接受的话：

1. 完成 `divide` 函数以通过前四个测试。
2. 通过完成 `result_with_list` 和 `list_of_results` 函数来通过剩余的测试。

请查看 Rust Book 的[迭代器](https://doc.rust-lang.org/stable/book/ch13-02-iterators.html)章节和[迭代器文档](https://doc.rust-lang.org/stable/std/iter/)。

向下滚动可以找到第2部分的小提示，进一步向下滚动可以找到主要提示。玩得开心 :-)

<div class="hint">
  当无法进行均匀除法时，<code>divide</code> 函数需要返回正确的错误。
</div>

<div class="hint">
<code>division_results</code> 变量需要被收集到一个集合类型中。

`result_with_list` 函数需要返回一个单一的 `Result`，其中成功情况是整数向量，失败情况是 `DivisionError`。

`list_of_results` 函数需要返回一个 `Result` 向量。
</div>
## 实现 TryFrom

`TryFrom` 是一种简单且安全的类型转换，在某些情况下可能以受控方式失败。基本上，这与 `From` 类型相同。主要区别在于，这应该返回一个 `Result` 类型而不是目标类型本身。 您可以在 https://doc.rust-lang.org/std/convert/trait.TryFrom.html 上阅读更多相关信息。

您的任务是完成此实现，并返回内部类型为 `Color` 的 `Ok` 结果。您需要为包含三个整数的元组、包含三个整数的数组和整数切片创建实现。

请注意，元组和数组的实现将在编译时检查，但切片实现需要检查切片的长度！ 还请注意，正确的 RGB 颜色值必须是 0..=255 范围内的整数。

<div class="hint">请按照任务中提供的步骤进行操作。您也可以使用此 <a href="https://doc.rust-lang.org/std/convert/trait.TryFrom.html">示例</a>。</div>

<div class="hint">提示：标准库中是否有 `TryFrom` 的实现可以同时进行所需的整数转换并检查输入范围？</div>

<div class="hint">查看测试用例，了解需要返回哪些错误变体。</div>

<div class="hint">您可以使用 `Result` 的 `map_err` 或 `or` 方法来转换错误。</div>

<div class="hint">如果您想在解决方案中使用 `?` 运算符来传播错误，您可能希望查看这篇 <a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html">文章</a>。</div>

**挑战**：您能否使 `TryFrom` 实现对多种整数类型通用？
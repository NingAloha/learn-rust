## 自定义错误

在库代码中使用诸如 `Box<dyn error::Error>` 这样的全局错误类型并不推荐，因为调用者可能希望根据错误内容做出决定，而不是简单地打印或进一步传播错误。在这里，我们定义一种自定义错误类型，使调用者能够决定当我们的函数返回错误时下一步该做什么。

让测试通过！如果你遇到困难，请阅读提示。

<div class="hint">此练习使用了
<a href="course://Error Handling/Errors Primer/Positive Nonzero Integer">前一个任务</a>中的<code>PositiveNonzeroInteger</code>的完整版本。</div>

<div class="hint">你可以在<code>impl ParsePosNonzeroError</code>中创建另一个函数。

例如，可以利用从 `ParseIntError` 到 `ParsePosNonzeroError` 的构建，这可能会由 `parse` 函数返回。
</div>

<div class="hint">在注释要求你更改的那行下面，有一个示例，展示了如何使用<code>map_err()</code>方法将<code>Result</code>中的一种错误类型转换为另一种。尝试对<code>parse()</code>返回的<code>Result</code>使用类似的方法。你可能会使用<code>?</code> 操作符来提前返回函数结果，或者使用<code>match</code>表达式，或者可能还有其他方法！</div>

<div class="hint">在<code>std::result</code>的 <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err">文档</a>中阅读更多关于<code>map_err()</code>的信息。
</div>
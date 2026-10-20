## 高级错误

还记得在之前的练习中，我们使用了多个映射函数，以便可以使用 `map_err()` 将底层错误转换成自定义错误类型吗？如果我们可以直接使用 `?` 操作符呢？

让这段代码编译成功！在 `main()` 函数的每个情况下完成代码，以便返回适当的错误。注意注释，如果遇到困难，可以查看提示。

<div class="hint">
解析代码现已在 <code>FromStr</code> 特征的实现中。注意，解析代码直接使用了 <code>?</code>，没有调用 <code>map_err()</code>。你应该完成一个部分实现的 <code>From</code> 特征示例。
</div>

<div class="hint">
细节：<code>?</code> 操作符在错误类型上调用 <code>From::from()</code>，以将其转换为周围函数的返回类型的错误类型。
</div>
<div class="hint">
你需要编写另一个具有不同输入类型的 <code>From</code> 实现。
</div>
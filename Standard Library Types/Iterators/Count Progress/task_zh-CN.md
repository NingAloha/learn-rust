## 计数进度

Rustling 的进度是通过哈希映射进行建模的。练习的名称是键，进度是值。创建了两个计数函数来计算具有给定进度的练习数量。这些计数函数使用命令式风格的 for 循环。使用迭代器重新创建此计数功能。

让代码编译并通过测试。

<div class="hint">步骤 1： <code>std::iter::Iterator</code> 特性的文档包含许多有用的方法。
</div>

<div class="hint">
步骤 2：从 <code>count_stack</code> 中返回 0 以使代码编译以测试计数。
</div>

<div class="hint"><code>count_stack</code> 中的 <code>stack</code> 变量是 HashMap 的一个切片。需要将其转换为迭代器以使用迭代器方法。
</div>

<div class="hint"><code>fold</code> 方法在 <code>count_stack</code> 函数中可能很有用。</div>
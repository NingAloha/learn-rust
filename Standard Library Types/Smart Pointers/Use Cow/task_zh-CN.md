## 使用 `Cow`

本次练习探讨了 `Cow`，或称为写时克隆（Clone-On-Write）类型。
`Cow` 是一种写时克隆的智能指针。
它可以封装并提供对借用数据的不变访问，并在需要变更或所有权时懒惰地克隆数据。
该类型旨在通过 `Borrow` 特性处理通用的借用数据。

首先查看 `abs_all` 函数：如果给定的切片包含负元素，它会对其进行修改。

在每一种情况（`case1`、`case2`、`case3` 和 `case4`）中的目标是决定对 `Cow::from` 和 `abs_all` 函数的调用结果是 `Cow::Borrowed` 还是 `Cow::Owned`。

<div class="hint">
确保你理解每一种情况中传递给 <code>Cow::from</code> 的具体内容。在这些情况下，所有权是否被转移？
</div>

<div class="hint">
那么在 <code>case3</code> 和 <code>case4</code> 中的向量如何，在 <code>Cow::from</code> 之后它是被借用还是拥有？
</div>

<div class="hint">
由于 <code>case4</code> 中的向量已经是拥有的，即使我们需要修改它，<code>Cow</code> 类型也不需要克隆它。
</div>

<div class="hint">查阅 <a href="https://doc.rust-lang.org/std/borrow/enum.Cow.html"><code>Cow</code> 类型的文档</a>。</div>
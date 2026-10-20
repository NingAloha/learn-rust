## 匹配结果

假设我们正在设计一个用代币购买物品的游戏。所有物品都需要5个代币，每当你购买物品时，还有1个代币的处理费。游戏玩家将输入他们想购买的物品数量，而 `total_cost` 函数将计算代币总数。然而，由于玩家输入的是数量，我们得到的是一个字符串，他们可能输入了任何东西，而不仅仅是数字！

现在，这个函数完全没有处理错误情况（而且也没有正确处理成功情况）。我们想做的是：如果我们对一个不是数字的字符串调用 `parse` 函数，该函数会返回一个 `ParseIntError`，在这种情况下，我们希望立即从我们的函数返回该错误，而不尝试乘法和加法。

至少有两种方法可以实现这一点，这两种方法都是正确的——但其中一种更短！向下滚动查看两种方法的提示。

<div class="hint">
  处理这种情况的一种方法是对 <code>item_quantity.parse::&lt;i32&gt()</code> 使用 <code>match</code> 匹配语句，其中的情况是 <code>Ok(something)</code> 和 <code>Err(something)</code>。
  然而，这种模式在 Rust 中非常常见，所以有一个 <code>?</code> 操作符几乎完成了你希望匹配语句为你做的事情！
  看看 <a href="https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator">错误处理章节的这一节</a> 并试试看！
</div>
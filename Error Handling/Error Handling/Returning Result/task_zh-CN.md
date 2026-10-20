## 返回 Result

如果传入一个空字符串，这个函数将拒绝生成要打印在名牌上的文本。 让它解释问题所在，而不是仅仅有时候返回 `None`，会更好。

<div class="hint">
为何不使用 <code>Result</code> 类型而不是 <code>Option</code>？
</div>

<div class="hint">
为了做出这个修改，你需要：

- 更新函数签名中的返回类型为 `Result<String, String>`，可以是变体 `Ok(String)` 和 `Err(String)`
- 修改函数体，在当前返回 `Some(stuff)` 的地方返回 `Ok(stuff)`
- 修改函数体，在当前返回 `None` 的地方返回 `Err(错误信息)`
</div>
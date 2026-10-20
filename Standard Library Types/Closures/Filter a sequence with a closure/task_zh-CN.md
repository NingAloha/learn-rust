## 任务：使用闭包过滤序列

你有一个包含 `i32` 元素的数组，一个 `cap`，以及一个计数器 `left_behind`。你的任务是完成作为参数传递给 `filter` 方法的闭包。闭包应该只保留严格小于 `cap` 的元素。同时，它还应计数所有被丢弃的元素。

<div class="hint">
阅读关于 <code>filter</code> 方法的文档，以理解闭包参数中的 <code>&&</code> 是什么意思。
</div>

<div class="hint">
你可能需要利用这样一个事实，即当 <code> condition || condition </code> 左侧为 <code>false</code> 时，才会评估右侧。
</div>

<div class="hint">
使用 <code>{...; condition}</code> 在评估某些逻辑表达式的同时执行一些操作。
</div>
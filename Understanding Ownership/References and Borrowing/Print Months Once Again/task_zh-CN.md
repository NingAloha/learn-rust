## 再次打印月份

这里发生了一些奇怪的事情。程序编译时没有任何问题。我们有 `fn print_months`，它借用了一个数组并打印其内容。这没有问题。那么 `fn print_months_reversed` 呢？它不是对 `months` 数组取得了所有权吗？为什么我们之后还能再次使用相同的 `months` 呢？实际上，并没有在这里转移所有权。数组的值被复制了。如果你运行程序，就会马上看到这一点。

重构此程序，通过在 `fn print_months_reversed` 中切换为可变引用来避免数组复制，并修正编译器错误。

<div class="hint">
第一步是更改 <code>fn print_months_reversed</code> 中参数的类型。
</div>

<div class="hint">
我们应该如何调用借用其参数为可变的方法？
</div>
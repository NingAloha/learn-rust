## 统一生命周期

既然编译器只是验证传递给注解参数和返回类型的引用，我们需要更改什么？

<div class="hint">
请记住，通用生命周期<code>'a</code>将获取等于<code>x</code>和<code>y</code>中较小生命周期的具体生命周期。
</div>

<div class="hint">
您可以至少采取两种途径来在保持内部块的同时实现所需的结果：

1. 将`string2`的声明移动，使其与`string1`一样长（`result`是如何声明的？）
2. 将`println!`移动到内部块中。

</div>
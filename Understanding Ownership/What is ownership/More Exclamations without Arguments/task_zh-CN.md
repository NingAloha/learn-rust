## 更多无参数的感叹

重构此代码，使得我们在`fn main`中不再创建字符串`Hello`，而是在`fn with_exclamation`中创建，并将新创建的字符串从`fn with_exclamation`传递给调用者。

<div class="hint">
  当您觉得有足够的指导时，请随时停止阅读 :)
  或者尝试做一步，然后修复由此产生的编译器错误！

因此，过程如下：
- 去掉在`main`中创建新字符串的第一行
- 因为现在`Hello`不存在了，我们不能将其传递给`with_exclamation`
- 因为我们不想向`with_exclamation`传递任何东西，它的签名应该反映出它不需要任何参数
- 既然我们不再在`main`中创建新字符串，我们需要在`with_exclamation`中创建一个新字符串，类似于我们在`main`中所做的方式
</div>
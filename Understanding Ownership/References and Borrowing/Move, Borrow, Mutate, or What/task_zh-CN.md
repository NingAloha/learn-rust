## 移动、借用、变异，或者其他

使我能够编译！除了函数的参数，不要改变任何东西。

<div class="hint">
注意，<code>fn get_last_char</code> 不应该获取所有权，
因为 <code>message</code> 在 <code>main</code> 函数中稍后还会使用。
相反，你可以使用引用来在那借用它。
</div>

<div class="hint">
注意，<code>fn uppercase_and_print</code> 是 <code>main</code> 中的最后一个调用，所以你可以安全地移动
<code>message</code> 到那里。因此，不需要传递引用。

还请注意，此函数会修改 `message` 变量。因此，你应该将其定义为可变的。
</div>
## 掌握IDE：管理条件

你能快速理解`main.rs`文件中的代码输出什么吗？

%IDE_NAME%拥有许多有用的功能，可以帮助你使代码更具可读性。

例如，让我们应用[德摩根律](https://en.wikipedia.org/wiki/De_Morgan%27s_laws)的**快速修复**。

将光标放在`&&`上，并使用&shortcut:ShowIntentionActions;快捷键查看可用的上下文操作。

![](image.png)

现在，你能快速理解`main.rs`文件中的代码输出什么吗？

<div class="hint">
  结果的if条件应该是 <br>
  <code>(number > 4 && number <= 9) || (number > 0 && number < 10)</code>
</div>

<div class="hint">
  代码输出“如果分支”
</div>

*注1*：`&&`和`||`是[短路逻辑运算符](https://en.wikipedia.org/wiki/Short-circuit_evaluation)

*注2*：尝试在你之前任务中的代码的不同地方使用&shortcut:ShowIntentionActions;。
%IDE_NAME%常常有关于如何改进代码的有用建议。
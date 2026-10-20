## 多个错误

此程序使用了上一个任务中完成的代码版本。 目前它不会编译！ 为什么？ 如果卡住，请使用提示。

<div class="hint">
在<code>main()</code>中产生了两种不同的<code>Result</code>类型，这些类型通过<code>?</code>运算符传播。 我们如何声明一个从<code>main()</code>返回的类型以允许两者共存？</div>

<div class="hint">另一个提示：在幕后，<code>?</code>运算符调用<code>From::from</code>将错误值转换为一个被框化的特征对象，即<code>Box&lt;dyn error::Error></code>，这是多态的——这意味着同一函数可以返回多种不同种类的错误，因为所有错误都表现相同，因为它们都实现了<code>error::Error</code>特征。 查看本<a href="course://Error Handling/Error Handling/Propagating Errors Limitations">任务</a>中的“传播错误的快捷方式：? 运算符”一节。</div>

<div class="hint">此练习使用了一些我们将在课程后期才会涉及的概念，如<code>Box</code>和<code>From</code>特征。目前深入理解它们并不重要，但如果你愿意，可以提前阅读。
在此处阅读更多关于框化错误的内容：
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html">这里</a>。</div>
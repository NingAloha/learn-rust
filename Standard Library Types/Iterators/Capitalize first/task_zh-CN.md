## 首字母大写

在本模块中，您将学习迭代器可以提供的一些独特优势。

步骤1. 完成 `capitalize_first` 函数以通过前两个测试。当输入为 "hello" 时应返回 "Hello"，当输入为空字符串时应返回空字符串。

步骤2. 将 `capitalize_first` 函数应用于字符串切片的切片。
返回字符串的向量。
`["hello", "world"]` -> `["Hello", "World"]`

步骤3. 将 `capitalize_first` 函数再次应用于字符串切片的切片。
返回单个字符串。
`["hello", " ", "world"]` -> `"Hello World"`

一如既往，有提示可供参考！

<div class="hint">
变量 <code>first</code> 是一个 <code>char</code>。需要将其大写并添加到
<code>c</code> 中剩余的字符以返回正确的 <code>String</code>。
<code>c</code> 中的剩余字符可以通过
<code>as_str</code> 方法视为字符串切片。
<code>char</code> 的文档包含许多有用的方法。
<a href="https://doc.rust-lang.org/std/primitive.char.html">https://doc.rust-lang.org/std/primitive.char.html</a>。</div>

<div class="hint">从切片创建一个迭代器。通过应用
<code>capitalize_first</code> 函数转换迭代值。记住要收集迭代器。</div>

<div class="hint">这与之前的解决方案非常相似。Collect 功能非常强大且通用。Rust 只需要知道所需的类型。</div>
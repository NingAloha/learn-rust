## 实现 FromStr

这实际上与 `TryFrom<&str>` 的工作方式几乎相同。此外，实施 `FromStr` 后，可以使用字符串的 `parse` 方法生成实现者类型的对象。你可以在 https://doc.rust-lang.org/std/str/trait.FromStr.html 上查看更多信息。

步骤:
1. 如果提供的字符串长度为 0，则返回错误。
2. 根据字符串中存在的逗号拆分字符串。
3. 拆分后应返回 2 个元素，否则返回错误。
4. 从拆分操作中提取第一个元素并将其用作名称。
5. 从拆分操作中提取另一个元素，将其解析为 `usize` 作为年龄，
就像 `"4".parse::<usize>()` 这样。
6. 如果在解析年龄时出现问题，则应返回错误。
   如果一切顺利，返回一个 `Person` 对象的 `Result`。

<div class="hint"> <code>FromStr</code> 的实现应返回一个包含 <code>Person</code> 对象的 <code>Ok</code>，
或者如果字符串无效，则返回包含字符串的 <code>Err</code>。 </div>

<div class="hint"> 这几乎就像从一个 exercise，但返回错误代替返回默认值。</div>

<div class="hint"> 提示：查看测试用例以了解要返回的错误变体。</div>

<div class="hint"> 另一个提示：你可以使用 <code>Result</code> 的 <code>map_err</code> 方法和一个函数
或闭包来封装 <code>parse::<usize></code> 的错误。</div>

<div class="hint"> 还有一个提示：如果你想在解决方案中使用 <code>?</code> 操作符传递错误，
你可能想查看 <a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html">这篇</a>文章。</div>
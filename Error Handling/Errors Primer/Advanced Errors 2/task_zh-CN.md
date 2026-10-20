## 高级错误处理 2

此练习展示了一些对于自定义错误类型有用的方法，尤其是为了使其他代码可以更有效地使用自定义错误类型。

让这个编译并使测试通过！如果卡住了，请查看提示。

步骤：
1. 确保 `ParseClimateError` 适合在 `main()` 中进行错误传播。
2. 完成对 `ParseClimateError` 的 `From` 的部分实现。
3. 使用下面的解析指南，在 `Climate` 的 `FromStr` 实现中处理缺失的错误情况。
4. 完成对 `ParseClimateError` 的 `Display` 的部分实现。

`Climate` 的解析器：
1. 将输入字符串分割成 3 个字段：`city`，`year`，`temp`。
2. 如果字符串为空或字段数量不正确，返回错误。
3. 如果城市名称为空，返回错误。
4. 将年份解析为 `u32`，如果解析失败返回错误。
5. 将温度解析为 `f32`，如果解析失败返回错误。
6. 返回包含完整的 `Climate` 值的 `Ok` 值。

添加或完成缺失的步骤。
不要忘记阅读代码中的注释。

<div class="hint">

让我们看看 `main.rs` 文件。`main` 函数的结果类型是 `Result<(), Box<dyn Error>>`。这意味着我们可以返回 `()`，或者*任何错误*。接下来的 `main` 函数中我们使用了错误传播：

```rust
  "Hong Kong,1999,25.7".parse::<Climate>()?
```

此代码传播了自定义错误 `ParseClimateError`。为了使其适合作为*任何错误*传递，我们需要将其做成一个 `Error`，如下所示：
```rust
impl Error for ParseClimateError {}
```

`Error`（`std::error::Error`）是一个代表错误值基本期望的特性。我们将在[课程后面的部分](course://Generic%20Types,%20Traits,%20and Lifetime/Traits/Traits)更详细地讨论特性。
</div>

<div class="hint">
在<code>Error</code>实现中不必实现任何方法。（某些方法有默认提供的实现。）
</div>

<div class="hint">查阅测试以确定在某些错误条件下需要生成哪些错误变体（以及哪些错误消息文本）。</div>

<div class="hint">
您可能会发现以下页面是有用的参考资料：
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/define_error_type.html">1</a>,
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html">2</a>, 
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/wrap_error.html">3</a>.
</div>

**挑战**: 有一个测试被标记为 `#[ignore]`。您能补充缺少的代码使其通过吗？您可能需要查阅标准库文档中的某个特性以获取更多提示。
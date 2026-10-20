## 使用 Clippy 作为外部 Linter

`Clippy` 工具是一组用于分析代码的 lint，以帮助您捕捉常见错误并改进您的 Rust 代码。要使用 `Clippy` 作为外部 linter，请按照[先前任务](course://Introduction/Getting started/External Linter)的说明操作，但这一次选择 **Clippy** 而不是 **Cargo Check**。

检查 Clippy 的建议并应用它们来解决练习。请注意，在这个任务中，**Check** 按钮实际上并不会检查任何内容。这里请自行判断！

<div class="hint">
Rust 在<a href="https://doc.rust-lang.org/stable/std/f32/consts/index.html">Rust 标准库</a>中存储了任何长或无限精度数学常数的最高精度版本。

我们可能会尝试使用自己的近似值来表示某些数学常数，但 Clippy 识别出这些不精确的数学常数是一种潜在的错误来源。在编译输出中查看 `clippy` 警告的建议，并使用来自 `std::f32::consts` 的适当替换常数。
</div>
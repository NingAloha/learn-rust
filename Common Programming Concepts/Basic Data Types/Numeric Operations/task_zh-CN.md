## 数值运算

Rust 支持你对所有数字类型所需的基本数学运算：加法、减法、乘法、除法和取余。以下代码展示了如何在 `let` 语句中使用每种运算：

```rust
fn main() {
    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;

    // 取余
    let remainder = 43 % 5;
}
```

这些语句中的每个表达式都使用了一个数学运算符，并计算出单个值，然后绑定到一个变量。[附录 B](https://doc.rust-lang.org/stable/book/appendix-02-operators.html) 包含 Rust 提供的所有运算符列表。
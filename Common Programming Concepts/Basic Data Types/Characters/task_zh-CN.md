## 字符类型

到目前为止，我们只处理了数字，但Rust也支持字母。Rust的`char`类型是语言中最基本的字母类型，以下代码展示了一种使用它的方法。（注意，`char`字面量用单引号表示，而字符串字面量用双引号。）

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart = '❤';
}
```

Rust的`char`类型大小为四个字节，表示一个Unicode标量值，这意味着它不仅可以表示ASCII。带重音的字母；中文、日文和韩文字符；表情符号；以及零宽度空格在Rust中都是有效的`char`值。Unicode标量值的范围从`U+0000`到`U+D7FF`和`U+E000`到`U+10FFFF`（包含）。然而，“字符”在Unicode中并不是真正的概念，因此您对“字符”的人类直觉可能与Rust中的`char`不符。我们将在第8章的[“用字符串存储UTF-8编码文本”](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings)详细讨论这个话题。
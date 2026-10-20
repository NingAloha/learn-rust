## 字符串切片

一个 _字符串切片_ 是对 `String` 的一部分的引用，看起来像这样：

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

这与获取整个 `String` 的引用相似，但多了一个 `[0..5]` 位。它不是对整个 `String` 的引用，而是对 `String` 的一部分的引用。

我们可以通过在方括号中指定范围来创建切片，格式为 `[起始索引..结束索引]`，其中 `起始索引` 是切片中的第一个位置，`结束索引` 是切片中的最后一个位置 + 1。在内部，切片数据结构存储了切片的起始位置和长度，这个长度对应于 `结束索引` 减去 `起始索引`。所以在 `let world = &s[6..11];` 的情况下，`world` 将是一个切片，包含一个指向 `s` 的第7个字节（从1开始计数）的指针，并且长度值为5。

图6以图表形式展示了这一点。

<img alt="world包含一个指向String s第6个字节的指针和长度5" src="https://doc.rust-lang.org/stable/book/img/trpl04-06.svg" class="center" style="width: 50%;">

##### 图6：字符串的一部分引用的字符串切片

在 Rust 的 `..` 范围语法中，如果你想从第一个索引（零）开始，你可以去掉两个点之前的值。换句话说，它们是相等的：

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

同理，如果你的切片包括了 `String` 的最后一个字节，你可以去掉后面的数字。这意味着它们是相等的：

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

你也可以去掉两个值来获取整个字符串的切片。因此，这些是相等的：

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

> 注意：字符串切片范围索引必须发生在有效的 UTF-8 字符边界上。如果你试图在多字节字符的中间创建字符串切片，程序将会报错并退出。为了介绍字符串切片的目的，在本节中我们仅假设 ASCII；关于 UTF-8 处理的更全面讨论请参考章节“用字符串存储 UTF-8 编码的文本”的 [“Storing UTF-8 Encoded Text with Strings”](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings) 部分。

结合所有这些信息，让我们重写 `first_word` 函数以返回一个切片。表示“字符串切片”的类型写作 `&str`：

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

我们以与第一个代码示例相同的方式获取单词末尾的索引，通过寻找第一个空格。当我们找到一个空格时，我们使用字符串的起始位置和空格的位置作为切片的起始和结束索引来返回一个字符串切片。

现在当我们调用 `first_word` 时，我们得到一个与底层数据关联的单一值。该值由一个指向切片起始点的引用和切片中元素的数量组成。

返回一个切片同样适用于 `second_word` 函数：

```rust
fn second_word(s: &String) -> &str {
```

我们现在拥有一个更直观的 API，更不容易出错，因为编译器会确保对 `String` 的引用始终有效。回想一下在程序“调用 `first_word` 函数，并然后更改 `String` 内容”的示例中的 bug，当我们得到了第一个单词的结束索引，然后清除了字符串，所以我们的索引无效？那个代码在逻辑上是错误的，但没有立即显示错误。如果我们继续尝试在清空的字符串上使用第一个单词索引，问题会晚些时候显现。切片使这个 bug 不可能发生，并提前通知我们代码存在问题。使用切片版本的 `first_word` 将抛出一个编译时错误：

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

这是编译器错误信息：

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 |
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here
```

回想一下借用规则，如果我们对某物进行不可变引用，就不能对其进行可变引用。因为 `clear` 需要截断 `String`，所以需要获取一个可变引用。Rust 不允许这么做，导致编译失败。Rust 不仅让我们的 API 更易于使用，还在编译时消除了整类错误！
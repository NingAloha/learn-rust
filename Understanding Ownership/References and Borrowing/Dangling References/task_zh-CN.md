## 悬空引用

在具有指针的编程语言中，很容易错误地创建一个_悬空指针_，即指向一个可能已被其他人使用的内存位置的指针，方法是释放一些内存，但保留了对该内存的指针。相比之下，在 Rust 中，编译器保证引用永远不会成为悬空引用：如果您对某些数据有引用，编译器将确保在对该数据的引用失效前，数据不会超出作用域。

让我们尝试创建一个悬空引用，Rust 会通过编译时错误来阻止：

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

这是错误信息：

```text
    error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                ^^^^^^^^

```

该错误信息提到一个我们尚未涉及的特性：生命周期。我们将在第 10 章详细讨论生命周期。但如果不考虑关于生命周期的部分，信息确实包含了该代码存在问题的关键：

```text
    this function's return type contains a borrowed value, but there is no value
    for it to be borrowed from.
```

让我们仔细看看我们的 `dangle` 代码在每个阶段究竟发生了什么：

```rust
fn dangle() -> &String { // dangle 返回一个对 String 的引用

    let s = String::from("hello"); // s 是一个新的 String

    &s // 我们返回对 String s 的引用
} // 在这里，s 超出作用域，被丢弃。它的内存消失了。
  // 危险！
```

由于 `s` 是在 `dangle` 内部创建的，当 `dangle` 的代码执行完毕时，`s` 将会被释放。但我们试图返回一个对它的引用。这意味着这个引用将指向一个无效的 `String`。这不好！Rust 不会让我们这样做。

这里的解决方案是直接返回 `String`：

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

这样不会有任何问题。所有权被转移出去，没有任何东西被释放。
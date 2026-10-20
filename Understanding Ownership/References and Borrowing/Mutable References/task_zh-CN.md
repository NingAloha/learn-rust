## 可变引用

那么，如果我们尝试修改我们借用的东西会发生什么呢？尝试运行下面代码片段中的代码。剧透：它不起作用！

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

##### 尝试修改借用的值

这是错误信息：

```text
    error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^ `some_string` 是一个 `&` 引用，所以它指向的数据不能被借用为可变的
```

正如变量默认是不可变的，引用也是如此。我们不允许修改我们引用的东西。

我们可以通过稍作调整来修复上述代码片段中的错误：

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

首先，我们必须将 `s` 改为 `mut`。然后我们必须用 `&mut s` 创建一个可变引用，并用 `some_string: &mut String` 接收一个可变引用。
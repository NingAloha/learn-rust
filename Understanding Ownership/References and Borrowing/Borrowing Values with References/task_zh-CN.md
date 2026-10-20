## 使用引用借用值

前一个任务中元组代码的问题是，我们必须将`String`返回给调用函数，以便在调用`calculate_length`之后仍然可以使用`String`，因为`String`被移动到了`calculate_length`中。

以下是如何定义和使用`calculate_length`函数，其中参数为对象的引用而不是值的所有权：

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("'{}' 的长度是 {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

首先，注意变量声明和函数返回值中的所有元组代码都消失了。其次，注意我们将`&s1`传递给`calculate_length`，并且在其定义中，我们接收的是`&String`而非`String`。

这些和号是_引用_，它们允许你引用某个值而无需拥有它。图5显示了一个示意图。

<img alt="&amp;String s 指向 String s1" src="https://doc.rust-lang.org/stable/book/img/trpl04-05.svg" class="center">

##### 图5：&String s 指向 String s1 的示意图

> 注意：使用`&`进行引用的相反操作是_解引用_，通过解引用运算符`*`实现。我们将在第8章看到一些解引用运算符的用法，并在第15章讨论解引用的细节。

让我们更仔细地看看这里的函数调用：

```rust
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
```

`&s1`语法让我们创建一个_引用_，它_指向_`s1`的值而不占有它。因为它不占有，所以它指向的值在引用超出作用域时不会被丢弃。

同样，函数的签名使用`&`来表示参数`s`的类型是引用。让我们加上一些解释性注释：

```rust
fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它不拥有所引用的值，
  // 所以不会发生任何事情。
```

变量`s`有效的作用域与任何函数参数的作用域相同，但我们不会在引用超出作用域时丢弃它指向的内容，因为我们不拥有它。当函数的参数是引用而不是实际值时，我们不需要返回值以归还所有权，因为我们从未拥有过所有权。

我们称以引用作为函数参数为_借用_。就像在现实生活中，如果某人拥有某物，你可以向他们借用它。当你完成后，你必须归还。
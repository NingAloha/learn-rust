## 变量和数据交互的方式：移动

在 Rust 中，多个变量可以以不同的方式与同一个数据交互。让我们看看以下代码片段中使用整数的示例。

```rust
let x = 5;
let y = x;
```

##### 将变量 x 的整数值赋给 y

我们可能可以猜到这段代码在做什么：“将值 `5` 绑定到 `x`；然后将 `x` 中的值复制一份并绑定到 `y`。”现在我们有两个变量，`x` 和 `y`，它们的值均为 `5`。这确实发生了，因为整数是具有已知固定大小的简单值，这两个 `5` 的值被压入栈中。

现在让我们看看 `String` 版本：

```rust
let s1 = String::from("hello");
let s2 = s1;
```

这段代码看起来与之前的代码非常相似，因此我们可能会推测它的工作方式是一样的：即第二行会复制 `s1` 中的值并绑定到 `s2`。但实际上不是这样。

请查看图 1 以了解 `String` 的内部工作原理。一个 `String` 由三个部分组成，显示在左侧：一个指向保存字符串内容的内存的指针，一个长度和一个容量。这组数据存储在栈上。右侧是保存内容的堆内存。

<img alt="内存中的 String" src="https://doc.rust-lang.org/stable/book/img/trpl04-01.svg" class="center" style="width: 50%;">

##### 图 1: 一个包含值 "hello" 的 String 绑定到 s1 的内存表示

长度是 `String` 当前使用的内存字节数。容量是 `String` 从分配器那里接收到的总内存字节数。长度和容量之间的区别很重要，但在这个上下文中无关紧要，所以目前可以忽略容量。

当我们将 `s1` 赋给 `s2` 时，复制的是 `String` 数据，这意味着我们复制了栈上的指针、长度和容量。我们没有复制指针指向的堆上的数据。换句话说，内存中的数据表示如图 2。

<img alt="s1 和 s2 指向相同的值" src="https://doc.rust-lang.org/stable/book/img/trpl04-02.svg" class="center" style="width: 50%;">

##### 图 2: 一个变量 s2 的内存表示，拥有 s1 的指针、长度和容量的副本

内存表示 _不_ 像图 3 所示，这是 Rust 复制堆数据时内存的可能样子。如果 Rust 这样做，操作 `s2 = s1` 在运行时性能方面可能非常昂贵，如果堆上的数据很大时。

<img alt="s1 和 s2 到两个位置" src="https://doc.rust-lang.org/stable/book/img/trpl04-03.svg" class="center" style="width: 50%;">

##### 图 3: 如果 Rust 同时复制堆数据，s2 = s1 的另一种可能性

之前，我们说当变量超出作用域时，Rust 自动调用 `drop` 函数并清理该变量的堆内存。但图 2 显示两个数据指针指向同一位置。这是个问题：当 `s2` 和 `s1` 超出作用域时，它们会尝试同时释放相同的内存。这就是所说的 _双重释放_ 错误，是我们之前提到的一种内存安全错误。两次释放内存可能导致内存损坏，甚至可能导致安全漏洞。

为了确保内存安全，Rust 在这种情况下还有一个细节。Rust 不试图复制分配的内存，而是认为 `s1` 不再有效，因此当 `s1` 超出作用域时，Rust 不需要释放任何东西。看看当你在创建 `s2` 之后试图使用 `s1` 时会发生什么；它不会工作：

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

你会收到这样的错误，因为 Rust 阻止你使用无效的引用：

```text
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 | 
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
```

如果你在其他语言中听说过 _浅拷贝_ 和 _深拷贝_ 的术语，这种不复制数据而仅复制指针、长度和容量的做法可能听起来像是在做浅拷贝。但因为 Rust 还使第一个变量失效，这种方式不是浅拷贝，而是称为 _移动_。在这个示例中，我们会说 `s1` 被 _移动_ 到 `s2`。所以实际发生的事情如图 4 所示。

<img alt="s1 移动到 s2" src="https://doc.rust-lang.org/stable/book/img/trpl04-04.svg" class="center" style="width: 50%;">

##### 图 4: s1 被使无效后的内存表示

这样就解决了我们的问题！在只有 `s2` 有效时，当它超出作用域时，它将唯一地释放内存，然后我们就完成了。

此外，这还暗示了一个设计选择：Rust 绝不会自动创建数据的“深”拷贝。因此，任何 _自动_ 复制都可以假定其在运行时性能方面是廉价的。
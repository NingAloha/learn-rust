### 闭包类型推断和注解

函数和闭包还有其他的不同之处。闭包通常不需要像 `fn` 函数那样注解参数或返回值的类型。在函数中需要类型注解，因为它们是显式接口的一部分，会暴露给用户。明确地定义这个接口对确保所有人都同意函数所使用和返回的值的类型是很重要的。但闭包并不是以这种暴露的接口形式使用的：它们被存储在变量中，可以在不命名和不暴露给我们库的用户的情况下使用。

闭包通常是短小的，并且只在狭窄的上下文中相关，而不是在任何随意的场景中。在这些有限的上下文中，编译器可以推断参数类型和返回类型，类似于它能够推断大多数变量的类型（在少数情况下，编译器也需要进行闭包类型注解）。

和变量一样，我们可以在希望增加显式性和清晰度的情况下添加类型注解，尽管这会使代码比严格需要的更加冗长。为闭包注解类型的样子如下所示：

```rust
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

##### 在闭包中添加参数和返回值类型的可选类型注解的示例

添加类型注解后，闭包的语法看起来更类似于函数的语法。以下是一个将参数加 1 的函数定义与具有相同行为的闭包的语法的垂直对比。我们添加了一些空格以对齐相关部分。这说明了闭包的语法是如何类似于函数语法的，除了使用竖线和可选语法的数量：

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

第一行显示了一个函数定义，第二行显示了一个完全注解的闭包定义。第三行移除了闭包定义中的类型注解，第四行移除了括号，因为闭包体只有一个表达式时它是可选的。这些都是有效的定义，调用时将产生相同的行为。要使 `add_one_v3` 和 `add_one_v4` 能够编译，必须调用闭包，因为类型将从它们的使用中推断出来。

闭包定义对每个参数和返回值将推断出一个具体类型。例如，下述是一个简短闭包的定义，它仅返回作为参数接收到的值。这个闭包并不常用，仅供此示例之用。注意我们没有在定义中添加任何类型注解：如果我们随后尝试用 `String` 作为第一次调用的参数，`u32` 作为第二次调用的参数，我们将会得到一个错误。

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

##### 试图用两种不同的类型调用类型被推断的闭包的示例

编译器给出了这样的错误：

```console
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |                             ^- help: try using a conversion method: `.to_string()`
  |                             |
  |                             expected struct `String`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `closure-example` due to previous error
```

第一次用 `String` 值调用 `example_closure` 时，编译器推断 `x` 的类型和闭包的返回类型为 `String`。这些类型随后被锁定在 `example_closure` 中，如果尝试用不同类型使用相同闭包，我们会得到一个类型错误。
### 结构体数据的所有权

在本节的第一个代码片段中定义的 `User` 结构体中，我们使用了拥有所有权的 `String` 类型，而不是字符串切片类型 `&str`。这是一个有意的选择，因为我们希望这个结构体的实例拥有其所有数据，并且这些数据在整个结构体有效的期间内都保持有效。

结构体可以存储由其他地方拥有的引用，但这样做需要使用 Rust 的一个特性——*生命周期*，我们将在“泛型类型、特征和生命周期”这一章中讨论。生命周期确保结构体引用的数据在结构体期间内是有效的。假如你试图在结构体中存储一个没有指定生命周期的引用，像这样，这将无法工作：

```rust,ignore,does_not_compile
 struct User {
     username: &str,
     email: &str,
     sign_in_count: u64,
     active: bool,
 }

 fn main() {
     let user1 = User {
         email: "someone@example.com",
         username: "someusername123",
         active: true,
         sign_in_count: 1,
     };
 }
```

编译器将会抱怨需要生命周期标注：

```console
 $ cargo run
    Compiling structs v0.1.0 (file:///projects/structs)
 error[E0106]: missing lifetime specifier
  --> src/main.rs:2:15
   |
 2 |     username: &str,
   |               ^ expected named lifetime parameter
   |
 help: consider introducing a named lifetime parameter
   |
 1 | struct User<'a> {
 2 |     username: &'a str,
   |

 error[E0106]: missing lifetime specifier
  --> src/main.rs:3:12
   |
 3 |     email: &str,
   |            ^ expected named lifetime parameter
   |
 help: consider introducing a named lifetime parameter
   |
 1 | struct User<'a> {
 2 |     username: &str,
 3 |     email: &'a str,
   |

 error: aborting due to 2 previous errors

 For more information about this error, try `rustc --explain E0106`.
 error: could not compile `structs`

 To learn more, run the command again with --verbose.
```

在“泛型类型、特征和生命周期”这一章中，我们将讨论如何修复这些错误，这样你就可以在结构体中存储引用。但目前，我们将通过使用像 `String` 这样的拥有所有权的类型而不是像 `&str` 这样的引用来修复此类错误。
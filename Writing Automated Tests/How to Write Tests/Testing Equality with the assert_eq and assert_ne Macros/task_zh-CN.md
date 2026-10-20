### 使用 assert_eq! 和 assert_ne! 宏测试相等性

测试功能的一种常见方法是将待测试代码的结果与您期望代码返回的值进行比较，以确保它们相等。您可以使用 `assert!` 宏，并传递一个使用 `==` 运算符的表达式来做到这一点。然而，这是一个如此常见的测试，以至于标准库提供了一对宏——`assert_eq!` 和 `assert_ne!`——以更方便地执行此测试。这些宏分别比较两个参数是否相等或不等。如果断言失败，它们还会打印这两个值，从而更容易看出测试失败的原因；相反，`assert!` 宏只指出 `==` 表达式的值为 `false`，而不是导致 `false` 值的具体值。

在以下代码片段中，我们编写了一个名为 `add_two` 的函数，该函数将 `2` 加到其参数上并返回结果。然后，我们使用 `assert_eq!` 宏测试此函数。

```rust
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_adds_two() {
            assert_eq!(4, add_two(2));
        }
    }
```

##### 使用 assert_eq! 宏测试函数 add_two 的示例

让我们检查一下，看它通过了！

```text
    running 1 test
    test tests::it_adds_two ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

我们传给 `assert_eq!` 宏的第一个参数 `4`，与调用 `add_two(2)` 的结果相等。该测试行是 `test tests::it_adds_two ... ok`，并且 `ok` 表示我们的测试通过了！

让我们在代码中引入一个错误，看看使用 `assert_eq!` 失败的测试是什么样子。将 `add_two` 函数的实现更改为增加 `3`：

```rust
    pub fn add_two(a: i32) -> i32 {
        a + 3
    }
```

再次运行测试：

```text
running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

我们的测试找到了这个错误！`it_adds_two` 测试失败，显示消息 `assertion failed: '(left == right)'` 并指出 `left` 是 `4` 而 `right` 是 `5`。这条消息很有用，有助于我们开始调试：意味着 `assert_eq!` 的 `left` 参数是 `4`，而 `right` 参数，也就是我们 `add_two(2)`，是 `5`。

请注意，在某些语言和测试框架中，用于断言两个值相等的函数的参数被称为 `expected` 和 `actual`，我们指定参数的顺序是重要的。然而，在 Rust 中，它们被称为 `left` 和 `right`，我们指定的期望值与待测代码产生的值的顺序并不重要。我们可以将此测试中的断言写为 `assert_eq!(add_two(2), 4)`，这将导致一个失败消息，显示 `assertion failed: '(left == right)'`，其中 `left` 为 `5`，`right` 为 `4`。

如果我们给 `assert_ne!` 宏提供的两个值不相等，则会通过测试；如果它们相等，则测试失败。此宏在我们不确定一个值会是什么，但我们知道代码按我们预期功能运行时绝对不会是什么的时候最有用。例如，我们正在测试一个保证以某种方式更改其输入的函数，但输入的更改方式取决于运行测试的那一天，此时最好的断言可能是函数的输出不等于输入。

在底层，`assert_eq!` 和 `assert_ne!` 宏分别使用运算符 `==` 和 `!=`。当断言失败时，这些宏使用调试格式打印其参数，这意味着被比较的值必须实现 `PartialEq` 和 `Debug` trait。所有的基本类型和大多数标准库类型都实现了这些 trait。对于您定义的结构体和枚举，您需要实现 `PartialEq` 以断言这些类型的值是否相等或不相等。您还需要实现 `Debug` 以便在断言失败时打印这些值。因为这两个 trait 都是可派生 trait，如在章节“结构体”的“示例结构体”部分中调用“添加注解以派生 `Debug` trait 并使用调试格式打印 `Rectangle` 实例”所述，通常这只需要在您的结构体或枚举定义中添加 `#[derive(PartialEq, Debug)]` 注释即可。有关这些和其他可派生 trait 的更多详细信息，请参阅附录 C，[“可派生的 Traits,”](https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html)。
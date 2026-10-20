### 使用 assert! 宏检查结果

由标准库提供的 `assert!` 宏在你希望确保测试中的某些条件评估为 `true` 时非常有用。我们给 `assert!` 宏一个评估为布尔值的参数。如果值是 `true`，`assert!` 宏不执行任何操作，测试通过。如果值是 `false`，`assert!` 宏调用 `panic!` 宏，导致测试失败。使用 `assert!` 宏帮助我们检查代码是否如我们期望的那样工作。

在"结构体/方法语法"一章中的"在 `Rectangle` 上实现 `can_hold` 方法，该方法将另一个 `Rectangle` 实例作为参数"的列表中，我们使用了一个 `Rectangle` 结构体和 `can_hold` 方法，下面将再次重复这些内容。让我们将这段代码放在 _src/lib.rs_ 文件中，并使用 `assert!` 宏为其编写一些测试。

```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
```

##### 使用 "结构体/方法语法" 一章中的 `Rectangle` 结构体及其 `can_hold` 方法的示例

`can_hold` 方法返回一个布尔值，这意味着它是 `assert!` 宏的完美用例。在下面的代码示例中，我们通过创建一个宽为8，高为7的 `Rectangle` 实例，并断言它可以容纳一个宽为5，高为1的另一个 `Rectangle` 实例来编写一个测试来验证 `can_hold` 方法。

```rust
   #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn larger_can_hold_smaller() {
            let larger = Rectangle { width: 8, height: 7 };
            let smaller = Rectangle { width: 5, height: 1 };

            assert!(larger.can_hold(&smaller));
        }
    }
```

##### 测试 `can_hold` 的示例，检查较大的矩形是否确实可以容纳较小的矩形

注意，我们在 `tests` 模块中添加了一行新代码：`use super::*;`。`tests` 模块是一个常规模块，遵循我们在"模块"（“模块和宏”章节）介绍的可见性规则。由于 `tests` 模块是内模块，我们需要将外部模块中的测试代码引入内模块的作用域。我们在这里使用了一个 glob，因此外部模块中定义的任何内容都可供这个 `tests` 模块使用。

我们将测试命名为 `larger_can_hold_smaller`，并创建了我们所需的两个 `Rectangle` 实例。然后我们调用了 `assert!` 宏，并传递给它调用 `larger.can_hold(&smaller)` 的结果。这个表达式应该返回 `true`，所以我们的测试应该通过。让我们来看结果！

```text
    running 1 test
    test tests::larger_can_hold_smaller ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

通过了！让我们再添加一个测试，这次断言较小的矩形不能容纳较大的矩形：

```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn larger_can_hold_smaller() {
            // --snip--
        }

        #[test]
        fn smaller_cannot_hold_larger() {
            let larger = Rectangle { width: 8, height: 7 };
            let smaller = Rectangle { width: 5, height: 1 };

            assert!(!smaller.can_hold(&larger));
        }
    }
```

因为在此情况下 `can_hold` 函数的正确结果是 `false`，所以我们需要在传递给 `assert!` 宏之前对结果进行取反。因此，如果 `can_hold` 返回 `false`，我们的测试就会通过：

```text
    running 2 tests
    test tests::smaller_cannot_hold_larger ... ok
    test tests::larger_can_hold_smaller ... ok

    test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

两次测试都通过了！现在让我们看看当我们在代码中引入一个错误时，测试结果会如何。让我们在 `can_hold` 方法的实现中用小于号代替比较宽度时的大于号：

```rust
    // --snip--

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width < other.width && self.height > other.height
        }
    }
```

现在运行测试会产生如下结果：

```text
    running 2 tests
    test tests::smaller_cannot_hold_larger ... ok
    test tests::larger_can_hold_smaller ... FAILED

    failures:

    ---- tests::larger_can_hold_smaller stdout ----
    thread 'tests::larger_can_hold_smaller' panicked at 'assertion failed:
    larger.can_hold(&smaller)', src/lib.rs:22:9
    note: Run with `RUST_BACKTRACE=1` for a backtrace.

    failures:
        tests::larger_can_hold_smaller

    test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

我们的测试捕捉到了这个错误！因为 `larger.width` 是8而 `smaller.width` 是5，所以 `can_hold` 中的宽度比较现在返回 `false`：8 不是小于 5。
## 添加自定义失败消息

您还可以通过 `assert!`、`assert_eq!` 和 `assert_ne!` 宏的可选参数添加自定义消息，以便与失败消息一起打印。在 `assert!` 后指定的参数或在 `assert_eq!` 和 `assert_ne!` 后指定的两个参数之后的任何参数都会传递给 `format!` 宏（关于 `format!` 宏的讨论可参考本书第 8 章的“[使用 `+` 运算符或 `format!` 宏的拼接](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro)”一节），因此您可以传递一个包含 `{}` 占位符的格式字符串以及想要填入占位符的值。自定义消息对于记述断言的意义很有用；当测试失败时，您将更好地了解代码的问题所在。

例如，假设我们有一个函数用于按名字迎接人，并且我们希望测试传递给函数的名字是否出现在输出中：

```rust
    pub fn greeting(name: &str) -> String {
        format!("Hello {}!", name)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn greeting_contains_name() {
            let result = greeting("Carol");
            assert!(result.contains("Carol"));
        }
    }
```

该程序的需求尚未确定，并且我们相当确定问候语开头的 `Hello` 文本将会发生变化。我们决定在需求更改时不更新测试，因此，与其检查从 `greeting` 函数返回值的精确相等性，不如仅断言输出是否包含输入参数的文本。

让我们通过将 `greeting` 更改为不包括 `name` 来引入一个错误，以查看此测试失败的样子：

```rust
    pub fn greeting(name: &str) -> String {
        String::from("Hello!")
    }
```

运行此测试会产生以下结果：

```text
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'main' panicked at 'assertion failed: result.contains("Carol")', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name
```

这个结果只是表明断言失败以及断言所在的行。在这种情况下，更有用的失败消息将打印我们从 `greeting` 函数获得的值。让我们更改测试函数，使用格式字符串中的占位符填入从 `greeting` 函数实际得到的值，生成一个自定义失败消息：

```rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```

现在，当我们运行测试时，我们将会得到更具信息量的错误消息：

```text
---- tests::greeting_contains_name stdout ----
thread 'main' panicked at 'Greeting did not contain name, value was `Hello!`', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

我们可以在测试输出中看到实际得到的值，这将帮助我们调试发生了什么，而不是我们预期会发生什么。
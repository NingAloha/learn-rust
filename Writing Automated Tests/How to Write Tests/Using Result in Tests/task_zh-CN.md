### 在测试中使用 Result<T, E>

到目前为止，我们编写的测试在失败时会触发 panic。我们也可以编写使用 `Result<T, E>` 的测试！以下是[之前任务中的一个测试](course://Writing Automated Tests/How to Write Tests/The Anatomy of a Test Function)，经过重写以使用 `Result<T, E>` 并返回 `Err` 而不是触发 panic：

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() -> Result<(), String> {
            if 2 + 2 == 4 {
                Ok(())
            } else {
                Err(String::from("two plus two does not equal four"))
            }
        }
    }
```

`it_works` 函数现在具有返回类型 `Result<(), String>`。在函数体中，我们不再调用 `assert_eq!` 宏，而是在测试通过时返回 `Ok(())`，在测试失败时返回包含 `String` 的 `Err`。

编写返回 `Result<T, E>` 的测试使您可以在测试体中使用问号运算符，这是一个可方便地编写如果内部任一操作返回 `Err` 变体时测试应失败的方法。

您不能在使用 `Result<T, E>` 的测试中使用 `#[should_panic]` 注解。相反，当测试应该失败时，您应直接返回一个 `Err` 值。

既然您已经了解多种编写测试的方法，我们来看看运行测试时发生了什么，并探索使用 `cargo test` 的不同选项。
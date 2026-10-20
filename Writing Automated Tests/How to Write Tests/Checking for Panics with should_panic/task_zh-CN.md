### 使用 `should_panic` 检查程序崩溃

除了检查我们的代码返回期望的正确值之外，确保我们的代码按预期处理错误情况也很重要。例如，考虑我们在章节“可恢复和不可恢复错误”的部分“是否应该崩溃”中创建的 `Guess` 类型，该类型将仅接受 1 到 100 之间的值。使用 `Guess` 的其他代码依赖于 `Guess` 实例仅包含 1 到 100 之间的值这一保证。我们可以编写一个测试，确保试图创建一个超出该范围的 `Guess` 实例会导致崩溃。

我们通过向测试函数添加另一个属性 `should_panic` 来实现这一点。此属性使得如果函数内部的代码崩溃，测试就会通过；如果函数内部的代码没有崩溃，测试将失败。

下面的代码片段展示了一个测试，它检查 `Guess::new` 的错误条件是否按照我们的预期发生。

```rust
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value 必须在 1 和 100 之间，但得到了 {}。", value);
            }

            Guess {
                value
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[should_panic]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
```

##### 测试某个条件将导致 `panic!` 的示例

我们在 `#[test]` 属性后面与测试函数之前放置 `#[should_panic]` 属性。让我们看看当这个测试通过时的结果：

```text
    运行 1 个测试
    test tests::greater_than_100 ... ok

    测试结果：ok。1 通过；0 失败；0 忽略；0 测量；0 过滤
```

看起来不错！现在让我们通过删除 `new` 函数的条件故意引入一个错误，使当值大于 100 时不再崩溃：

```rust
    // --代码剪切--

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1  {
                panic!("Guess value 必须在 1 和 100 之间，得到了 {}。", value);
            }

            Guess {
                value
            }
        }
    }
```

当我们运行“测试某个条件将导致 `panic!` 的示例”中的测试时，它将失败：

```text
运行 1 个测试
test tests::greater_than_100 ... 失败

失败详情：

---- tests::greater_than_100 stdout ----
注意：测试没有按期望崩溃

失败项：
    tests::greater_than_100

测试结果：失败。0 通过；1 失败；0 忽略；0 测量；0 过滤
```

在这种情况下，我们没有得到很有帮助的信息，但当我们查看测试函数时，我们看到它用 `#[should_panic]` 注释。我们得到的失败意味着测试函数中的代码没有引发崩溃。

使用 `should_panic` 的测试可能不够精确，因为它们仅表明代码引发了一些崩溃。即使测试由于不同于我们预期的原因引发崩溃，`should_panic` 测试也会通过。为了让 `should_panic` 测试更加精确，我们可以向 `should_panic` 属性添加可选的 `expected` 参数。测试工具将确保错误信息包含提供的文本。例如，请考虑 `Guess` 的修改代码（如下所示），其中 `new` 函数根据值是否太小或太大会引发不同的信息。

```rust
    // --代码剪切--

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!("Guess value 必须大于或等于 1，得到了 {}。",
                       value);
            } else if value > 100 {
                panic!("Guess value 必须小于或等于 100，得到了 {}。",
                       value);
            }

            Guess {
                value
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[should_panic(expected = "Guess value 必须小于或等于 100")]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
```

##### 测试某个条件将导致特定崩溃信息的示例

此测试将通过，因为我们在 `should_panic` 属性的 `expected` 参数中放置的值是 `Guess::new` 函数引发信息的一个子串。我们本可以指定我们期望的完整崩溃信息，在这个例子中是 `Guess value 必须小于或等于 100，得到了 200。` 你在 `should_panic` 的期望参数中选择指定什么取决于崩溃信息有多独特或者动态，以及你希望测试的精确程度。在这个例子中，崩溃信息的一个子串足以确保测试函数中的代码执行 `else if value > 100` 条件分支。

要查看当一个带有 `expected` 的 `should_panic` 测试失败时会发生什么，让我们再次通过交换 `if value < 1` 和 `else if value > 100` 块的内容来引入一个错误：

```rust
    if value < 1 {
        panic!("Guess value 必须小于或等于 100，得到了 {}。", value);
    } else if value > 100 {
        panic!("Guess value 必须大于或等于 1，得到了 {}。", value);
    }
```

这次当我们运行 `should_panic` 测试时，它将失败：

```text
运行 1 个测试
test tests::greater_than_100 ... 失败

失败详情：

---- tests::greater_than_100 stdout ----
主线程 'main' 在 'Guess value 必须大于或等于 1，得到了 200。' 崩溃，src/lib.rs:13:13
注意：使用 `RUST_BACKTRACE=1` 环境变量运行以显示堆栈跟踪
注意：崩溃信息不包含预期字符串
      崩溃信息：`"Guess value 必须大于或等于 1，得到了 200。"`,
 预期的子串：`"Guess value 必须小于或等于 100"`

失败项：
    tests::greater_than_100

测试结果：失败。0 通过；1 失败；0 忽略；0 测量；0 过滤
```

失败信息表明此测试确实如我们所期望的那样引发崩溃，但崩溃信息未包含预期的字符串 `Guess value 必须小于或等于 100`。我们在这种情况下得到的崩溃信息是 `Guess value 必须大于或等于 1，得到了 200。` 现在我们可以开始找出我们的错误在哪里了！
### 集成测试

在 Rust 中，集成测试是完全独立于你的库的。它们以和任何其他代码一样的方式使用你的库，这意味着它们只能调用你库的公共 API 中的函数。它们的目的是测试库的多个部分是否能够正确协同工作。单个代码单元单独工作正常，但在集成后可能出现问题，因此对集成代码的测试覆盖也很重要。要创建集成测试，你首先需要一个 *tests* 目录。

#### *tests* 目录

我们在项目目录的顶层创建了一个 *tests* 目录，就在 *src* 旁边。Cargo 知道要在这个目录中查找集成测试文件。然后，我们可以在这个目录中创建任意数量的测试文件，Cargo 会将每个文件编译为单独的 crate。

让我们看看一个集成测试。根据 *src/lib.rs* 文件中“测试私有函数”一节的代码，在 *tests* 目录中查看，其中有一个名为 *tests/integration_test.rs* 的文件，其代码如下所示。

```rust
use integration_tests;

#[test]
fn it_adds_two() {
    assert_eq!(4, integration_tests::add_two(2));
}
```

##### `integration_tests` crate 中一个函数的集成测试

我们在代码顶部添加了 `use integration_tests;`，这在单元测试中是不需要的。原因是 `tests` 目录中的每个文件都是一个单独的 crate，所以我们需要将我们的库引入每个测试 crate 的作用域。

我们不需要在 *tests/integration_test.rs* 中用 `#[cfg(test)]` 注释任何代码。Cargo 对 `tests` 目录有特殊处理，只在我们运行 `cargo test` 时编译此目录中的文件。现在运行 `cargo test`：

```text
Compiling integration_tests v0.1.0 
    Finished test [unoptimized + debuginfo] target(s) in 0.54s
     Running target/debug/deps/integration_tests-61f5d8d60ccbcc19
     
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_tests-d5df7484b111e79e

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests integration_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

输出的三个部分包括单元测试、集成测试和文档测试。单元测试部分与我们之前看到的一样：每个单元测试一行（我们在“测试私有函数”中添加的名为 `internal` 的测试）以及单元测试的汇总行。

集成测试部分以 `Running target/debug/deps/integration_tests-d5df7484b111e79e` 这一行开始（输出的哈希会有所不同）。接着是该集成测试中每个测试函数的一行，紧随其后的是集成测试结果的汇总行，然后是 `Doc-tests integration_tests` 部分开始。

类似于添加更多的单元测试函数会在单元测试部分增加更多的结果行，向集成测试文件添加更多的测试函数会使集成测试文件的该部分增加更多的结果行。每个集成测试文件都有其自己的部分，因此如果我们在 *tests* 目录中添加更多文件，会有更多的集成测试部分。

我们仍然可以通过指定测试函数的名称作为 `cargo test` 的参数来运行特定的集成测试函数。要运行特定集成测试文件中的所有测试，请使用 `cargo test` 的 `--test` 参数，并附上文件名（`cargo test --test integration_test`）：

```text
running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

此命令仅运行 *tests/integration_test.rs* 文件中的测试。
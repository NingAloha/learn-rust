## 集成测试中的子模块

随着你添加更多的集成测试，你可能希望在*tests*目录中创建多个文件来更好地组织它们。例如，你可以按测试功能对测试函数进行分组。如之前所述，*tests*目录中的每个文件都会被编译成一个独立的crate。

将每个集成测试文件视为一个独立的crate对于创建彼此独立的作用域是有用的，这更类似于最终用户使用你的crate的方式。然而，这意味着*tests*目录中的文件与*src*中的文件行为不同，正如你在“模块和宏/模块”中学习到的那样，可以将代码分隔成模块和文件。

*tests*目录中的文件不同的行为在你拥有一组可以在多个集成测试文件中使用的辅助函数且尝试按照“将模块分隔到不同文件中”章节中提到的步骤来将它们提取到一个公共模块时最为显著。例如，如果我们创建*tests/common.rs*并在其中放置一个名为`setup`的函数，我们可以在`setup`中添加一些我们希望在多个测试文件的多个测试函数中调用的代码：

```rust
pub fn setup() {
    // 这里会放置与你的库测试相关的设置代码
}
```

当我们再次运行测试时，即使这个文件不包含任何测试函数，也没有从任何地方调用`setup`函数，我们会在测试输出中看到一个新的关于*common.rs*文件的部分：

```text
   Compiling test_organization v0.1.0
    Finished test [unoptimized + debuginfo] target(s) in 0.81s
     Running target/debug/deps/test_organization-61f5d8d60ccbcc19

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/common-b5e4eefa9d201089

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-5843d720c5feeb7a

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests test_organization

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

让`common`出现在测试结果中并显示`running 0 tests`并不是我们想要的。我们只是想与其他集成测试文件共享一些代码。你将在下一节中学习如何避免`common`出现在测试输出中并正确组织测试。

为了避免`common`出现在测试输出中，我们将不创建*tests/common.rs*，而是创建*tests/common/mod.rs*。这是Rust也理解的一种备用命名约定。以这种方式命名文件告诉Rust不要将`common`模块视为一个集成测试文件。当我们将`setup`函数代码移动到*tests/common/mod.rs*并删除*tests/common.rs*文件时，测试输出中的这一部分将不再出现。*tests*目录子目录中的文件不再作为单独的crate编译，也不会在测试输出中出现部分。

在创建*tests/common/mod.rs*后，我们可以从任何的集成测试文件中将其作为模块使用。以下是一个在*tests/integration_test.rs*中的`it_adds_two`测试中调用`setup`函数的示例：

```rust,ignore
use test_organization_part_2;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_organization_part_2::add_two(2));
}
```

注意`mod common;`声明与我们在"将模块声明到_src/front_of_house.rs_"部分中展示的模块声明是一样的，然后在测试函数中，我们可以调用`common::setup()`函数。

在创建*tests/common/mod.rs*并从*tests/integration_test.rs*中的`it_adds_two`测试调用`setup`函数后的`cargo test`输出：

```text
Compiling submodules v0.1.0 
    Finished test [unoptimized + debuginfo] target(s) in 0.50s
     Running target/debug/deps/submodules-c44b35b673c8053d

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-31048908068047a2

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests submodules

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

##### 二进制crate的集成测试

如果我们的项目是一个仅包含*src/main.rs*文件且没有*src/lib.rs*文件的二进制crate，我们无法在*tests*目录中创建集成测试并通过`use`语句将*src/main.rs*文件中定义的函数引入作用域。只有库crate才会暴露其他crate可用的函数；二进制crate是用于独立运行的。

这是Rust项目提供二进制时，在*src/main.rs*文件中调用逻辑通常位于*src/lib.rs*文件内的一个原因。使用这种结构，集成测试*可以*通过`use`语句测试库crate，使重要的功能可用。如果重要的功能正常工作，*src/main.rs*文件中少量的代码也会正常工作，而这少量的代码不需要测试。
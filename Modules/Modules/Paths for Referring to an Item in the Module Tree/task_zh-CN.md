## 在模块树中引用项目的路径

为了让 Rust 知道在模块树中寻找项目的位置，我们使用 _路径_，就像在文件系统中导航时使用路径一样。如果我们想调用一个函数，需要知道它的路径。

路径可以有两种形式：

*   绝对路径：从 crate 根目录开始，使用 crate 名称或字面量 `crate`。
*   相对路径：从当前模块开始，使用 `self`、`super` 或当前模块中的标识符。

无论是绝对路径还是相对路径，都是由一个或多个标识符组成，并用双冒号（`::`）分隔。

让我们回到上一任务中第一个代码片段中的例子。如何调用 `add_to_waitlist` 函数？这相当于在问，`add_to_waitlist` 函数的路径是什么？在下面的代码片段中，我们通过删除一些模块和函数简化了代码。我们将展示两种从 crate 根目录中新函数 `eat_at_restaurant` 调用 `add_to_waitlist` 函数的方法。`eat_at_restaurant` 函数是我们库 crate 公共 API 的一部分，因此我们用 `pub` 关键字标记它。在“通过 `pub` 关键字暴露路径”部分，我们将详细说明 `pub`。注意，这个例子还不能编译；我们稍后会解释原因。

```rust
    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // 绝对路径
        crate::front_of_house::hosting::add_to_waitlist();

        // 相对路径
        front_of_house::hosting::add_to_waitlist();
    }
```

##### 使用绝对和相对路径调用 add_to_waitlist 函数

第一次在 `eat_at_restaurant` 中调用 `add_to_waitlist` 函数时，我们使用绝对路径。`add_to_waitlist` 函数与 `eat_at_restaurant` 定义在同一个 crate 中，这意味着我们可以使用 `crate` 关键字来开始绝对路径。

在 `crate` 之后，我们逐一包含后续模块，直到到达 `add_to_waitlist`。你可以想象一个具有相同结构的文件系统，我们可以指定路径 `/front_of_house/hosting/add_to_waitlist` 来运行 `add_to_waitlist` 程序；使用 `crate` 名称从 crate 根目录开始就像使用 `/` 从文件系统根目录开始一样。

第二次在 `eat_at_restaurant` 中调用 `add_to_waitlist` 时，我们使用相对路径。路径从 `front_of_house` 开始，它是在模块树中与 `eat_at_restaurant` 处于同一层次的模块的名称。这里在文件系统中相当于使用路径 `front_of_house/hosting/add_to_waitlist`。从名称开始意味着路径是相对的。

选择使用相对路径还是绝对路径是基于项目开发中的决策。该决策应取决于你是在更有可能单独移动项目定义代码，还是与使用项目的代码一起移动。例如，如果我们将 `front_of_house` 模块和 `eat_at_restaurant` 函数移动到一个名为 `customer_experience` 的模块中，我们需要更新 `add_to_waitlist` 的绝对路径，但相对路径仍然有效。然而，如果我们将 `eat_at_restaurant` 函数单独移动到一个名为 `dining` 的模块中，调用 `add_to_waitlist` 的绝对路径将保持不变，但相对路径需要更新。我们倾向于指定绝对路径，因为将代码定义和项目调用独立移动更为常见。

让我们尝试编译上面的代码片段，看看为什么它还不能编译！错误信息如下代码片段所示。

```text
Compiling Test_Rust_Project v0.1.0
error[E0603]: module `hosting` is private
 --> src/main.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^

error[E0603]: module `hosting` is private
  --> src/main.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^
```

##### 编译上述示例代码时出现的编译器错误

错误消息显示模块 `hosting` 是私有的。换句话说，我们有 `hosting` 模块和 `add_to_waitlist` 函数的正确路径，但 Rust 不允许我们使用它们，因为它无法访问私有部分。

模块不仅仅用于组织代码。它们还定义了 Rust 的 _隐私边界_：即外部代码不能知道、调用或依赖的实现细节的界限。因此，如果你想让某个项目（例如函数或结构体）变为私有，可以将其放入模块中。

Rust 中的隐私机制工作方式是，所有项目（函数、方法、结构体、枚举、模块和常量）默认为私有。父模块中的项目不能使用其子模块中的私有项目，但子模块中的项目可以使用其祖先模块中的项目。原因是子模块包裹并隐藏了它们的实现细节，但子模块可以看到其定义的上下文。继续餐厅的比喻，隐私规则就像餐厅的后台：后台的运作对餐厅顾客是私有的，但后台经理可以看到并操作餐厅中的任何事物。

Rust 选择让模块系统这样运作，默认隐藏内部实现细节。这样，你就知道可以更改哪些内部代码而不破坏外部代码。但你可以通过使用 `pub` 关键字将子模块代码的内部部分公开给外部祖先模块，从而使一个项目变为公共。
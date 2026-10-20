### 使用 pub 关键字公开路径

让我们回到上一个示例中告诉我们 `hosting` 模块是私有的错误。我们希望父模块中的 `eat_at_restaurant` 函数能够访问子模块中的 `add_to_waitlist` 函数，因此我们用 `pub` 关键字标记 `hosting` 模块，正如下列表所示。

```rust
    mod front_of_house {
        pub mod hosting {
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

##### 将 hosting 模块声明为 pub 以便从 eat_at_restaurant 中使用

不幸的是，上面的代码片段仍然会导致如下错误。

```text
正在编译 Test_Rust_Project v0.1.0
错误[E0603]: 函数 `add_to_waitlist` 是私有的
 --> src/main.rs:9:37
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^

错误[E0603]: 函数 `add_to_waitlist` 是私有的
  --> src/main.rs:12:30
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^
```

##### 从构建上述代码中获得的编译器错误

发生了什么？在 `mod hosting` 前加上 `pub` 关键字使模块变为公共的。通过这一更改，如果我们可以访问 `front_of_house`，我们也可以访问 `hosting`。但是，`hosting` 的_内容_仍然是私有的；将模块设为公共并不会使其内容变为公共。模块上的 `pub` 关键字仅允许其祖先模块中的代码引用它。

错误信息指出 `add_to_waitlist` 函数是私有的。隐私规则适用于结构体、枚举、函数和方法及模块。

让我们通过在其定义之前添加 `pub` 关键字来使 `add_to_waitlist` 函数也变为公共的，代码如下所示。

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // 绝对路径
        crate::front_of_house::hosting::add_to_waitlist();

        // 相对路径
        front_of_house::hosting::add_to_waitlist();
    }
```

##### 添加 pub 关键字到 mod hosting 和 fn add_to_waitlist 使我们可以从 eat_at_restaurant 调用函数

现在代码可以编译了！我们来看一下绝对路径和相对路径，并仔细检查为什么添加 `pub` 关键字使我们可以根据隐私规则使用这些路径中的 `add_to_waitlist`。

在绝对路径中，我们从 `crate` 开始，它是我们 crate 模块树的根。然后在 crate 根中定义了 `front_of_house` 模块。`front_of_house` 模块不是公有的，但因为 `eat_at_restaurant` 函数是在与 `front_of_house` 相同的模块中定义的（即 `eat_at_restaurant` 和 `front_of_house` 是兄弟模块），所以我们可以从 `eat_at_restaurant` 引用 `front_of_house`。接下来是标记为 `pub` 的 `hosting` 模块。我们可以访问 `hosting` 的父模块，因此我们可以访问 `hosting`。最后，`add_to_waitlist` 函数标记为 `pub`，并且我们可以访问其父模块，因此此函数调用有效！

在相对路径中，逻辑与绝对路径相同，除了第一步：而不是从 crate 根开始，路径从 `front_of_house` 开始。`front_of_house` 模块是在与 `eat_at_restaurant` 相同的模块中定义的，所以从定义 `eat_at_restaurant` 的模块开始的相对路径是可行的。然后，因为 `hosting` 和 `add_to_waitlist` 标记为 `pub`，路径的其余部分是有效的，这个函数调用是有效的！
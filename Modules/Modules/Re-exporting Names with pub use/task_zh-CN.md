### 使用 `pub use` 重新导出名称

当我们使用 `use` 关键字将名称引入作用域时，该名称在新作用域中是私有的。为了能够从其他代码中引用该名称（就像该名称已在该代码的作用域中定义一样），我们可以结合使用 `pub` 和 `use`。 这种技术称为 _重新导出_，因为我们不仅将项目引入作用域，还使该项目可供其他人引入到他们的作用域中。

接下来的示例展示了将任务开始代码中的根模块的 `use` 更改为 `pub use` 的代码。

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### 通过 pub use 使任何代码可以使用从新作用域中的名称

通过使用 `pub use`，外部代码现在可以使用 `hosting::add_to_waitlist` 调用 `add_to_waitlist` 函数。如果我们没有指定 `pub use`，则 `eat_at_restaurant` 函数可以在其作用域中调用 `hosting::add_to_waitlist`，但外部代码无法利用这个新路径。

重新导出在代码的内部结构与调用代码的程序员对领域的思考方式不同的情况下非常有用。例如，在这个餐厅的比喻中，经营餐厅的人会考虑“前台”和“后台”。但是，顾客在访问餐厅时可能不会以这种方式考虑餐厅的部分。使用 `pub use`，我们可以用一种结构编写代码，但公开时用不同的结构。这样做使得我们的库对在库上工作的程序员和调用库的程序员来说，都组织良好。
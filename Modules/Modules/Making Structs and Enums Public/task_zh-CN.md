### 使结构体和枚举为公有

我们也可以使用 `pub` 将结构体和枚举指定为公有，但还有一些额外的细节。如果我们在结构体定义前使用 `pub`，我们将使结构体公有，但结构体的字段仍然是私有的。我们可以根据具体情况决定是否将每个字段设为公有。在下面的例子中，我们定义了一个公有的 `back_of_house::Breakfast` 结构体，其中 `toast` 字段是公有的，但 `seasonal_fruit` 字段是私有的。这种模型适用于餐馆的场景，顾客可以选择随餐的面包类型，但厨师会根据季节和库存决定附带的水果。可用的水果变化很快，所以顾客无法选择水果，甚至看不到将得到哪种水果。

```rust
    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
    }

    pub fn eat_at_restaurant() {
        // 在夏天点一个包含黑麦土司的早餐
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // 改变我们想要的面包
        meal.toast = String::from("Wheat");
        println!("请给我 {} 土司", meal.toast);

        // 如果我们取消注释下一行，代码将无法编译；
        // 我们不能查看或修改随餐的季节性水果
        // meal.seasonal_fruit = String::from("blueberries");
    }
```

##### 包含一些公有字段和私有字段的结构体

因为 `back_of_house::Breakfast` 结构体中的 `toast` 字段是公有的，在 `eat_at_restaurant` 中我们可以使用点符号读取和写入 `toast` 字段。注意我们不能在 `eat_at_restaurant` 中使用 `seasonal_fruit` 字段，因为 `seasonal_fruit` 是私有的。尝试取消注释修改 `seasonal_fruit` 字段值的那一行，看看会出现什么错误！

另外注意，因为 `back_of_house::Breakfast` 有一个私有字段，结构体需要提供一个公有的关联函数来构造一个 `Breakfast` 实例（我们在这里把它命名为 `summer`）。如果 `Breakfast` 没有这样的函数，我们就无法在 `eat_at_restaurant` 中创建 `Breakfast` 的实例，因为无法在 `eat_at_restaurant` 中设置私有的 `seasonal_fruit` 字段的值。

相反，如果我们把一个枚举设置为公有，那么它的所有变体也都是公有的。我们只需要在 `enum` 关键字之前使用 `pub`，如下所示。

```rust
    mod back_of_house {
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    pub fn eat_at_restaurant() {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
```

##### 将枚举指定为公有使其所有变体公有

因为我们将 `Appetizer` 枚举设为公有，所以可以在 `eat_at_restaurant` 中使用 `Soup` 和 `Salad` 变体。如果枚举的变体不是公有的，枚举几乎没有什么用处；要求在每个情况下都为所有枚举变体标注 `pub` 会很麻烦，因此枚举变体默认是公有的。结构体在其字段不公有时常常也很有用，因此结构体字段遵循默认私有的通用规则，除非用 `pub` 标注。

还有一个关于 `pub` 的情况我们还没有涉及，那就是我们的最后一个模块系统特性：`use` 关键字。我们将先单独讲解 `use`，然后展示如何结合使用 `pub` 和 `use`。

_可以参考《Rust 程序设计语言》中的以下章节：[模块树中引用项目的路径](https://doc.rust-lang.org/stable/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#paths-for-referring-to-an-item-in-the-module-tree)_
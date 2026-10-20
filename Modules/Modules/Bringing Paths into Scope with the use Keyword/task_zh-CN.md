## 使用 `use` 关键字将路径引入作用域

目前我们编写的调用函数的路径似乎又长又重复。例如，在"使用模块树中路径指代项"的示例中，无论我们选择 `add_to_waitlist` 函数的绝对路径还是相对路径，每次想要调用 `add_to_waitlist` 时，我们都要同时指定 `front_of_house` 和 `hosting`。幸运的是，有一种方法可以简化这个过程。我们可以在作用域中引入一次路径，然后用 `use` 关键字像调用本地项一样调用该路径中的项。

在下面的示例中，我们将 `crate::front_of_house::hosting` 模块引入 `eat_at_restaurant` 函数的作用域，所以我们只需指定 `hosting::add_to_waitlist` 来调用 `eat_at_restaurant` 中的 `add_to_waitlist` 函数。

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### 使用 `use` 将模块引入作用域

在作用域中添加 `use` 和一个路径类似于在文件系统中创建符号链接。在 crate 根部添加 `use crate::front_of_house::hosting` 后，`hosting` 成为该作用域中的有效名称，就好像 `hosting` 模块已在 crate 根部定义一样。通过 `use` 引入作用域的路径也会检查隐私，就像其他路径一样。

你也可以使用 `use` 和相对路径将项引入作用域。下例展示了如何指定相对路径以获得与上述代码相同的行为。

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use self::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### 使用 `use` 和以 `self` 开头的相对路径将模块引入作用域

请注意，以这种方式使用 `self` 在未来可能不是必需的；这是语言中的一个不一致性，Rust 开发者正努力消除这种情况。

### 创建惯用的 `use` 路径

在第一个代码段中，你可能会想为什么我们要指定 `use crate::front_of_house::hosting`，然后在 `eat_at_restaurant` 中调用 `hosting::add_to_waitlist`，而不是将 `use` 路径一直指定到 `add_to_waitlist` 函数以实现相同的结果，如下面代码段所示。

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use crate::front_of_house::hosting::add_to_waitlist;

    pub fn eat_at_restaurant() {
        add_to_waitlist();
        add_to_waitlist();
        add_to_waitlist();
    }
```

##### 用 `use` 将 `add_to_waitlist` 函数引入作用域，这是不惯用的

虽然这两个代码段完成了相同的任务，第一个是以惯用的方式使用 `use` 将函数引入作用域。将函数的父模块引入作用域，我们需要在调用函数时指定父模块，这使得函数不是本地定义的很明确的同时仍然最小化了完整路径的重复。最后一个代码段的代码不清楚 `add_to_waitlist` 的定义位置。

另一方面，当用 `use` 引入结构体、枚举和其他项时，惯用的是指定完整路径。下面的例子展示了如何将标准库的 `HashMap` 结构体以惯用的方式引入二进制 crate 的作用域。

```rust
    use std::collections::HashMap;

    fn main() {
        let mut map = HashMap::new();
        map.insert(1, 2);
    }
```

##### 以惯用的方式将 `HashMap` 引入作用域

这种习惯并没有强烈的理由：它只是形成的惯例，人们已经习惯于以这种方式阅读和编写 Rust 代码。

这种惯例的例外是当我们用 `use` 语句引入两个同名项进入作用域时，因为 Rust 不允许那样做。下面的例子展示了如何引入两个同名但不同父模块的 `Result` 类型，并如何引用它们。

```rust
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {
    }

    fn function2() -> io::Result<()> {
    }
```

##### 使用父模块将两个同名类型引入同一作用域

正如你所见，使用父模块区分了两个 `Result` 类型。相反，如果我们指定 `use std::fmt::Result` 和 `use std::io::Result`，我们将在同一作用域内有两个 `Result` 类型，而 Rust 将不知道我们使用 `Result` 时指的是哪个。试试看，看看编译器会给出什么错误！
### 使用闭包捕获环境

我们要研究的闭包的第一个方面是，闭包可以从它们定义的环境中捕获值以供后续使用。这里有一个场景：一家T恤公司不时为他们的邮件列表中的某人赠送一件免费的T恤。邮件列表中的人可以选择在他们的档案中添加他们喜欢的颜色。如果被选中获得免费T恤的人在他们的档案中标明了喜欢的颜色，他们就会得到这种颜色的T恤。如果这个人没有指定喜欢的颜色，他们就会得到公司当前库存最多的颜色。

有很多方法可以实现这一点。对于这个例子，我们将使用一个名为`ShirtColor`的枚举，它有`Red`和`Blue`两个变体。公司的库存由一个名为`Inventory`的结构体表示，它有一个名为`shirts`的字段，包含一个代表当前库存T恤的`Vec<ShirtColor>`。`Inventory`上定义的方法`shirt_giveaway`获取得到免费T恤的人的可选T恤颜色偏好，并返回这个人将获得的T恤颜色。这在下面的列表中显示：

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

##### T恤公司赠品活动

`main`中定义的`store`库存中有两件蓝色T恤和一件红色T恤。然后，它调用`giveaway`方法，分别为一个喜欢红色T恤的用户和一个没有任何偏好的用户。运行这段代码时输出如下：

```console
$ cargo run
   Compiling shirt-company v0.1.0 (file:///projects/shirt-company)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/shirt-company`
The user with preference Some(Red) gets Red
The user with preference None gets Blue
```

再次说明，这段代码可以有多种实现方式，但这种方式使用了你已经学过的概念，除了`giveaway`方法体中使用了闭包。`giveaway`方法接受用户偏好`Option<ShirtColor>`并对其调用`unwrap_or_else`。标准库中的[`Option<T>`的`unwrap_or_else`方法](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.unwrap_or_else)被定义为接受一个参数：一个无参数的闭包，返回一个值`T`（与`Option<T>`中的`Some`变体存储的类型相同，在这里是`ShirtColor`）。如果`Option<T>`是`Some`变体，`unwrap_or_else`返回`Some`中存储的值。如果`Option<T>`是`None`变体，`unwrap_or_else`调用闭包并返回闭包返回的值。

这很有趣，因为我们传递了一个闭包，该闭包在当前的`Inventory`实例上调用`self.most_stocked()`。标准库不需要知道我们定义的`Inventory`或`ShirtColor`类型，或我们在这种场景下想要使用的逻辑。闭包捕获了对`self` `Inventory`实例的不可变引用，并将其与我们指定的代码一起传递给`unwrap_or_else`方法。函数无法以这种方式捕获它们的环境。
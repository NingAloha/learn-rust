## 定义和实例化结构体

结构体类似于元组，在“常见编程概念”一章中有讨论。与元组一样，结构体的各个部分可以是不同的类型。与元组不同的是，您将为每个数据片段命名，以便明确这些值的含义。由于为这些数据命名，结构体比元组更灵活：您不必依赖数据的顺序来指定或访问实例的值。

要定义一个结构体，我们需要输入关键字`struct`并命名整个结构体。结构体的名称应描述被组合在一起的数据片段的重要性。然后，在花括号中，我们定义数据片段的名称和类型，这些片段称为*字段*。例如，下面的示例展示了一个存储用户账户信息的结构体。

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

#### `User`结构体的定义

在定义结构体后要使用它，我们通过为每个字段指定具体值来创建该结构体的*实例*。我们通过声明结构体的名称，然后添加包含`key: value`对的花括号来创建实例，其中`key`是字段的名称，`value`是我们想存储在这些字段中的数据。不必按照在结构体中声明它们的顺序指定字段。换句话说，结构体定义就像是该类型的通用模板，而实例则用特定数据填充该模板来创建类型的值。例如，我们可以像下面这样声明一个特定的用户。

```rust
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
```

#### 创建一个`User`结构体的实例

要从结构体中获取特定值，我们可以使用点号表示法。如果我们只想要这个用户的电子邮件地址，可以在我们想使用此值的任何地方使用`user1.email`。如果实例是可变的，我们可以通过使用点号表示法和在特定字段中赋值来更改值。下面的代码展示了如何更改可变`User`实例的`email`字段的值。

```rust
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
```

#### 更改`User`实例的`email`字段的值

请注意，整个实例必须是可变的；Rust不允许我们仅标记某些字段为可变。与任何表达式一样，我们可以在函数体的最后一个表达式中构造结构体的新实例，以便隐式返回该新实例。

下面的代码展示了一个`build_user`函数，该函数返回具有给定电子邮件和用户名的`User`实例。`active`字段获取值`true`，`sign_in_count`获取值`1`。

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

#### 一个`build_user`函数，接收电子邮件和用户名并返回一个`User`实例

为函数参数使用与结构体字段相同的名称是有意义的，但重复`email`和`username`字段名称和变量有点繁琐。如果结构体有更多字段，重复每个名称会更加令人烦恼。幸运的是，有一种方便的简写！
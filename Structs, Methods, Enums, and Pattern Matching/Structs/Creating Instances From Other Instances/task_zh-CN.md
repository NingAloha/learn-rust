### 使用结构体更新语法从其他实例创建实例

通常，创建一个新的结构体实例时，会用到大部分旧实例的值，只更改某些值。这可以通过*结构体更新语法*来实现。

首先，下面的代码展示了如何在没有使用更新语法的情况下，在`user2`中创建一个新的`User`实例。我们为`email`和`username`设置了新值，但其他值仍然使用了我们在本节第二个代码示例中创建的`user1`中的相同值。

```rust
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
```

#### 使用`user1`的一些值创建新的`User`实例

使用结构体更新语法，我们可以用更少的代码实现同样的效果，如下所示。语法`..`表示那些没有显式设置的剩余字段应具有给定实例中字段的相同值。

```rust
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
```

#### 使用结构体更新语法为`User`实例设置新的`email`和`username`值，但使用`user1`变量中实例的其余字段的值

上述代码也在`user2`中创建了一个实例，其`email`和`username`具有不同的值，但`active`和`sign_in_count`字段与`user1`相同。
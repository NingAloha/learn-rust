### 当变量和字段同名时使用字段初始化简写

因为上述代码中的参数名和结构体字段名完全相同，我们可以使用*字段初始化简写*语法重新编写`build_user`，这样它的行为完全相同，但不会重复`email`和`username`，如下所示。

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

#### 使用字段初始化简写的 `build_user` 函数，因为 `email` 和 `username` 参数与结构体字段同名

在这里，我们正在创建一个名为`User`的结构体的新实例，它有一个名为`email`的字段。我们希望将`email`字段的值设置为`build_user`函数中的`email`参数的值。由于`email`字段和`email`参数同名，我们只需写`email`而不是`email: email`。
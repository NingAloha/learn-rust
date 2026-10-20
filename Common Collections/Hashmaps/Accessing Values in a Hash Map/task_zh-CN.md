### 访问哈希映射中的值

我们可以通过向 `get` 方法提供键来从哈希映射中获取一个值，如下所示。

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
```

#### 访问存储在哈希映射中的蓝队得分

这里，`score` 将拥有与蓝队相关联的值，结果将是 `Some(&10)`。结果被包裹在 `Some` 中，因为 `get` 返回一个 `Option<&V>`；如果哈希映射中没有该键的值，`get` 将返回 `None`。程序需要以我们在“枚举”章节中介绍的方式之一来处理 `Option`。

我们可以使用 `for` 循环以类似于向量的方式迭代哈希映射中的每个键/值对：

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
```

这段代码将以任意顺序打印每对：

```text
Yellow: 50
Blue: 10
```
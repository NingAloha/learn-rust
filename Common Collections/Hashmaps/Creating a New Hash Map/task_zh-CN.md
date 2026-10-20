### 创建新的哈希映射

你可以使用 `new` 创建一个空的哈希映射，并使用 `insert` 添加元素。在下面的代码中，我们跟踪两个队伍的得分，它们的名字是蓝队和黄队。蓝队初始得分为10分，黄队初始得分为50分。

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

#### 创建一个新的哈希映射并插入一些键和值

注意，我们需要首先从标准库的集合部分 `use` `HashMap`。在我们常用的三种集合中，这是最少使用的一种，因此在预导入中不会自动引入到作用域中。哈希映射在标准库中的支持也较少；例如，没有构建它们的内置宏。

与向量类似，哈希映射将其数据存储在堆上。这个 `HashMap` 的键类型是 `String`，值类型是 `i32`。与向量类似，哈希映射是同质的：所有的键必须是相同的类型，所有的值也必须是相同的类型。

另一种构造哈希映射的方法是使用迭代器和 `collect` 方法，将包含键和值的元组的向量转换为哈希映射。我们将在“迭代器和闭包”章的“迭代器”部分更详细地讨论迭代器及其相关方法。`collect` 方法将数据收集到多种集合类型中，包括 `HashMap`。例如，如果我们有两个分别代表队伍名称和初始得分的向量，我们可以使用 `zip` 方法创建一个包含元组的向量，其中“Blue”与10配对，依此类推。然后，我们可以使用 `collect` 方法将那个元组的向量转换为哈希映射，如下所示。

```rust
use std::collections::HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let mut scores: HashMap<_, _> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();
```

#### 从队伍列表和得分列表创建哈希映射

这里需要类型标注 `HashMap<_, _>`，因为可以 `collect` 成多种不同的数据结构，如果不指定，Rust 不知道你想要哪种结构。然而，对于键和值类型的参数，我们使用下划线，并且 Rust 可以根据向量中数据的类型推断出哈希映射中包含的类型。在上面的代码中，键的类型将是 `String`，值的类型将是 `i32`，这与本节第一部分的类型一致。
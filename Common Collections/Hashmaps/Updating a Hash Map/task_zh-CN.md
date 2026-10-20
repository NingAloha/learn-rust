### 更新哈希映射

虽然键和值的数量是可增长的，但每个键一次只能与一个值关联。当你想在哈希映射中更改数据时，你必须决定如何处理当一个键已经有值的情况。你可以用新值替换旧值，完全忽略旧值。你可以保留旧值并忽略新值，仅在键*没有*值的时候添加新值。或者你可以将旧值和新值结合起来。让我们来看看如何进行这些操作！

#### 覆盖值

如果我们在哈希映射中插入一个键和值，然后用不同的值再次插入相同的键，关联的值会被替换。尽管下面的代码调用了两次`insert`，但哈希映射只会包含一个键/值对，因为我们两次插入的都是蓝队键的值。

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
```

##### 替换存储在特定键中的值

这段代码将打印出`{"Blue": 25}`。原来的值`10`被覆盖了。

#### 仅在键没有值时插入值

经常需要检查特定键是否有值，如果没有，就为其插入一个值。哈希映射有一种特殊的API叫做`entry`，它将你要检查的键作为参数。`entry`方法的返回值是一个叫做`Entry`的枚举，表示一个可能存在或不存在的值。假设我们想要检查黄队的键是否有值。如果没有，我们想插入值50，蓝队也一样。使用`entry` API，代码如下：

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
```

##### 使用entry方法仅在键尚无值时插入

`Entry`上的`or_insert`方法定义为返回该键对应值的可变引用，如果键存在，否则将参数插入为新值并返回新值的可变引用。这一技术比我们自己编写逻辑要简洁得多，并且与借用检查器配合得更好。

运行上述代码将打印出`{"Yellow": 50, "Blue": 25}`。第一次调用`entry`会为黄队插入键和值50，因为黄队尚未有值。第二次调用`entry`不会更改哈希映射，因为蓝队已经有值25。

#### 根据旧值更新值

哈希映射的另一个常见用例是查找键的值，然后根据旧值更新它。例如，下面的代码展示了统计文本中每个单词出现次数的方法。我们使用哈希映射将单词作为键并递增值以记录我们看到该单词的次数。如果这是我们第一次看到一个单词，我们将首先插入值0。

```rust
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
```

##### 使用存储单词和计数的哈希映射统计单词出现次数

这段代码将打印出`{"world": 2, "hello": 1, "wonderful": 1}`。`or_insert`方法实际上返回这个键值的一个可变引用（`&mut V`）。在这里，我们将这个可变引用存储在`count`变量中，因此要赋值给该值，必须首先使用星号（`*`）解引用`count`。可变引用在`for`循环结束时会超出作用域，因此所有这些更改是安全并且符合借用规则的。
## 用 for 循环遍历数组

您可以使用 `while` 结构来遍历集合（例如数组）的元素。例如，让我们看以下代码：

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```
##### 使用 while 循环遍历集合中每个元素的示例

在这里，代码通过数组中的元素逐步计数。它从索引 `0` 开始，然后循环直到到达数组的最后一个索引（即当 `index < 5` 不再为真时）。运行此代码将打印数组中的每个元素：

```text
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

如预期，终端中出现了数组的所有五个值。即使 `index` 在某个时候会达到 `5` 的值，但循环会在尝试从数组中获取第六个值之前停止执行。

但这种方法容易出错；如果索引长度不正确，我们可能会导致程序崩溃。它也很慢，因为编译器为每个元素在每次循环中执行条件检查而添加了运行时代码。

作为更简洁的替代方案，您可以使用 `for` 循环，并为集合中的每个项目执行一些代码。一个 for 循环看起来像这样：

```rust
fn main() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("the value is: {}", element);
    }
}
```
##### 使用 for 循环遍历集合中每个元素的示例

当我们运行此代码时，将会看到与前面代码片段相同的输出。更重要的是，我们现在增加了代码的安全性，并消除了可能由于越过数组末尾或未遍历足够远而遗漏一些项目而导致的错误的可能性。

例如，在前面的代码片段中，如果您将 `a` 数组的定义更改为包含四个元素，但忘记更新条件为 `while index < 4`，代码会崩溃。而使用 `for` 循环，如果您更改数组中值的数量，就不需要记得更改任何其他代码。
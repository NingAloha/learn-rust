## 无重复代码

在下一个代码片段中，我们将用于查找最大数字的代码提取到一个名为 `largest` 的函数中。与本节第一个示例中的代码不同，它只能在一个特定列表中查找最大数字，而此程序可以在两个不同的列表中查找最大数字。

```rust
fn largest(list: &[i32]) -> &i32 {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
  let result = largest(&number_list);
  println!("The largest number is {}", result);
}
```

#### 用抽象代码查找两个列表中的最大数字

`largest` 函数有一个叫 `list` 的参数，它表示我们可能传入函数的任何具体的 `i32` 值的切片。结果是，当我们调用该函数时，代码会运行在我们传入的特定值上。

总而言之，这里是我们从第二个列表变更到第三个列表所采取的步骤：

1. 确认重复代码。
2. 将重复代码提取到函数体中，并在函数签名中指定这些代码的输入和返回值。
3. 更新重复代码的两个实例以调用该函数。

接下来，我们将使用这些相同的步骤与泛型结合以不同方式减少代码重复。同样，函数体可以操作抽象的 `list` 而不是具体值，泛型允许代码操作抽象类型。

例如，假设我们有两个函数：一个用于查找 `i32` 值切片中的最大项，另一个用于查找 `char` 值切片中的最大项。我们该如何消除这种重复呢？让我们找出答案！
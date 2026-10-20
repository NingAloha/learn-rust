## 重复的代码片段

在深入了解泛型语法之前，让我们先看看如何通过提取函数来去除不涉及泛型类型的重复代码。然后我们将应用这种技术来提取一个泛型函数！就像识别出重复代码以提取到函数中一样，您会开始识别可以使用泛型的重复代码。

为了在两个不同的数字列表中找到最大的数字，我们可以复制上述代码，并在程序的两个不同位置使用相同的逻辑，如下所示。

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("最大的数字是 {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("最大的数字是 {}", largest);
}
```

### 查找*两个*数字列表中最大数字的代码

虽然这段代码可以正常工作，但复制代码既繁琐又容易出错。当我们想要更改代码时，还必须在多个地方更新它。
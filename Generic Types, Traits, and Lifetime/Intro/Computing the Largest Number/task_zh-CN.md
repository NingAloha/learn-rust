## 计算最大数

考虑一个简短的程序，该程序在列表中查找最大数，如下面的代码片段所示。

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

#### 查找数字列表中最大数的代码

这段代码将整数列表存储在变量`number_list`中，并将列表中的第一个数字放入名为`largest`的变量中。然后，它遍历列表中的所有数字，如果当前数字大于存储在`largest`中的数字，它会替换该变量中的数字。但是，如果当前数字小于或等于到目前为止看到的最大数字，变量不会改变，代码将继续处理列表中的下一个数字。在考虑列表中的所有数字后，`largest`应该保存最大数字，在这个例子中是100。
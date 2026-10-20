### 使用闭包捕获其环境

现在我们已经介绍了迭代器，我们可以通过使用 `filter` 迭代器适配器来演示一个闭包捕获其环境的常见用法。迭代器上的 `filter` 方法接受一个闭包，该闭包从迭代器中获取每个项目并返回一个布尔值。如果闭包返回 `true`，该值将包含在 `filter` 产生的迭代器中。如果闭包返回 `false`，该值将不包含在结果迭代器中。

在下面的代码片段中，我们使用 `filter` 和一个从其环境中捕获 `shoe_size` 变量的闭包来迭代 `Shoe` 结构体实例的集合。它将仅返回指定尺码的鞋子。

```rust
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter()
            .filter(|s| s.size == shoe_size)
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn filters_by_size() {
            let shoes = vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 13, style: String::from("sandal") },
                Shoe { size: 10, style: String::from("boot") },
            ];

            let in_my_size = shoes_in_my_size(shoes, 10);

            assert_eq!(
                in_my_size,
                vec![
                    Shoe { size: 10, style: String::from("sneaker") },
                    Shoe { size: 10, style: String::from("boot") },
                ]
            );
        }
    }
```

##### 使用捕获 shoe_size 的闭包的 filter 方法

`shoes_in_my_size` 函数接收一组鞋子和一个鞋码作为参数。它返回一个仅包含指定尺码鞋子的向量。

在 `shoes_in_my_size` 的主体中，我们调用 `into_iter` 以创建一个拥有向量的迭代器。然后，我们调用 `filter` 将该迭代器适配为一个新迭代器，该新迭代器仅包含闭包返回 `true` 的元素。

闭包从环境中捕获 `shoe_size` 参数，并将其值与每个鞋子的尺码进行比较，仅保留指定尺码的鞋子。最后，通过调用 `collect` 将改编后的迭代器返回的值收集到一个由函数返回的向量中。

测试显示，当我们调用 `shoes_in_my_size` 时，我们只会得到与我们指定的值相同尺码的鞋子。
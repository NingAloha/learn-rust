### 在函数定义中

当定义使用泛型的函数时，我们会在函数签名中放置泛型，这里通常是指定参数和返回值的数据类型的位置。这样做使我们的代码更灵活，为函数的调用者提供更多功能，同时避免代码重复。

继续我们的 `largest` 函数，下面的代码展示了两个函数，它们都在切片中寻找最大的值。

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
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

    let result = largest_i32(&number_list);
    println!("最大的数字是 {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("最大的字符是 {}", result);
}
```

#### 两个函数，它们只在名称和签名中的类型不同

`largest_i32` 函数是我们在上一节中提取的代码段中的函数，用于查找切片中的最大 `i32`。`largest_char` 函数用于查找切片中的最大 `char`。这两个函数体的代码是相同的，因此我们可以通过在单个函数中引入泛型类型参数来消除重复。

为了给我们将要定义的新函数参数化类型，我们需要像为函数的值参数命名一样命名类型参数。可以使用任何标识符作为类型参数名。但我们会使用 `T`，因为按照惯例，Rust 中的参数名很短，通常仅用一个字母，并且 Rust 的类型命名惯例是驼峰式。`T` 是“type”的缩写，是大多数 Rust 程序员的默认选择。

当我们在函数体中使用参数时，必须在签名中声明参数名，以便编译器知道该名字的含义。同样，当我们在函数签名中使用类型参数名时，必须在使用之前声明类型参数名。要定义泛型 `largest` 函数，请在函数名和参数列表之间的尖括号 `<>` 中声明类型名称，如下所示：

```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```

我们将这个定义理解为：函数 `largest` 对某种类型 `T` 是泛型的。这个函数有一个名为 `list` 的参数，这是一个类型为 `T` 的值的切片。`largest` 函数将返回一个对与相同类型 `T` 的值的引用。

下面的代码段展示了结合使用泛型数据类型的 `largest` 函数定义。代码也展示了如何用 `i32` 值的切片或 `char` 值的切片调用该函数。请注意，这段代码还不能编译，但我们会在本章后面修正它。

```rust,ignore,does_not_compile
fn largest<T>(list: &[T]) -> &T {
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
    println!("最大的数字是 {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("最大的字符是 {}", result);
}
```

#### 使用泛型类型参数定义的 `largest` 函数，但尚未编译

如果我们现在编译这段代码，将会收到这样的错误：

```console
error[E0369]: 二元运算符 `>` 不能用于类型 `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: 考虑限制类型参数 `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ^^^^^^^^^^^^^^^^^^^^^^
```

该说明提到了 `std::cmp::PartialOrd`，这是一种*特征*。我们将在下一节讨论特征。现在，这个错误指出，`largest` 的主体无法对 `T` 可以是的所有可能类型工作。因为我们希望在主体中比较 `T` 类型的值，所以只能使用其值可以被排序的类型。为了启用比较，标准库提供了 `std::cmp::PartialOrd` 特征，可以在类型上实现（有关该特征的更多信息，请参见附录 C）。您将在下一课的“作为参数的特征”部分了解如何指定一个泛型类型有特定特征的细节，但首先让我们探索其他使用泛型类型参数的方法。
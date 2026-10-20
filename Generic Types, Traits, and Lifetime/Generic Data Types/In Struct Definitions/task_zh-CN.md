### 在结构体定义中

我们也可以在一个或多个字段中使用`<>`语法为结构体定义一个泛型类型参数。下面的代码展示了如何定义一个`Point<T>`结构体，以保存任意类型的`x`和`y`坐标值。

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

#### 一个`Point<T>`结构体，保存类型`T`的`x`和`y`值

在结构体定义中使用泛型的语法与在函数定义中使用的相似。首先，我们在结构体名称后面的尖括号中声明类型参数的名称。然后，我们可以在结构体定义中使用泛型类型，而无需指定具体的数据类型。

注意，因为我们只使用了一个泛型类型来定义`Point<T>`，这意味着`Point<T>`结构体是某种类型`T`的泛型，字段`x`和`y`都是相同类型的，不管这个类型是什么。如果我们创建一个`Point<T>`实例，其中的值类型不同，如下所示，我们的代码将无法编译。

```rust,ignore,does_not_compile
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

#### 字段`x`和`y`必须是相同的类型，因为两者具有相同的泛型数据类型`T`。

在这个例子中，当我们将整数值5赋给`x`时，我们告知编译器对于这个`Point<T>`实例，泛型类型`T`将是一个整数。然后当我们为`y`指定为4.0时，由于我们已经定义它与`x`具有相同的类型，我们会收到如下类型不匹配的错误：

```console
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integer, found floating-point number
```

要定义一个`Point`结构体，其中`x`和`y`都是泛型但可以是不同的类型，我们可以使用多个泛型类型参数。例如，在下面的列表中，我们可以将`Point`的定义更改为对类型`T`和`U`进行泛型化，其中`x`是类型`T`，`y`是类型`U`。

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

#### 一个`Point<T, U>`对两种类型进行泛型化，因此`x`和`y`可以是不同类型的值

现在展示的所有`Point`实例都是允许的！你可以在定义中使用任意多个泛型类型参数，但使用过多会使代码难以阅读。当你在代码中需要许多泛型类型时，这可能表明代码需要重构成较小的部分。
### 有不同数据的枚举值

使用枚举而不是结构体还有另一个优势：每个变体可以有不同类型和数量的关联数据。第四版的IP地址总是有四个数值组成，每个数值在0到255之间。如果我们想将`V4`地址存储为四个`u8`值，同时仍然将`V6`地址表示为一个`String`值，使用结构体是无法实现的。枚举可以轻松处理这种情况：

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

我们展示了几种不同的方法来定义数据结构以存储第四版和第六版IP地址。然而，事实证明，想要存储IP地址并编码它们的类型是如此常见，[标准库有一个可以使用的定义！][IpAddr]<!-- ignore --> 让我们看看标准库是如何定义`IpAddr`的：它具有我们定义和使用过的精确枚举和变体，但它将地址数据以两种不同的结构体形式嵌入在变体中，这些结构体对于每个变体的定义是不同的：

[IpAddr]: https://doc.rust-lang.org/std/net/enum.IpAddr.html

```rust
struct Ipv4Addr {
    // --省略--
}

struct Ipv6Addr {
    // --省略--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

此代码说明了你可以将任何类型的数据放入枚举变体中：字符串、数值类型或结构体等。你甚至可以包含另一个枚举！而且，标准库类型的复杂程度往往和你自己设想的差不了多少。

注意，尽管标准库包含了`IpAddr`的定义，我们仍可以创建和使用我们自己的定义而不会产生冲突，因为我们没有引入标准库的定义到我们的作用域中。我们会在[后面](course://Modules/Modules/Bringing Paths into Scope with the use Keyword)谈论如何把类型引入作用域。
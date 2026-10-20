### 带数据的枚举值

使用枚举有更多的优势。考虑到我们的 IP 地址类型，目前我们没有办法存储真正的 IP 地址*数据*，我们只知道它是什么*类型*。鉴于您在上一章中刚刚学习了结构体，您可以如下代码片段所示来解决这个问题。

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

#### 使用结构体存储 IP 地址的数据和 `IpAddrKind` 变体

在这里，我们定义了一个结构体 `IpAddr`，它有两个字段：一个是类型为 `IpAddrKind`（我们之前定义的枚举）的 `kind` 字段，另一个是类型为 `String` 的 `address` 字段。我们有两个此结构体的实例。第一个，`home`，其 `kind` 的值是 `IpAddrKind::V4`，与之关联的地址数据是 `127.0.0.1`。第二个实例，`loopback`，其 `kind` 值为 `IpAddrKind` 的另一变体 `V6`，关联的地址是 `::1`。我们使用结构体将 `kind` 和 `address` 值打包在一起，因此现在变体与值关联。

我们可以通过直接将数据放入每个枚举变体，而不是在结构体内嵌套枚举，以更简洁的方式表示相同的概念。这个新的 `IpAddr` 枚举定义表明，`V4` 和 `V6` 变体都将有关联的 `String` 值：

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

我们直接将数据附加到枚举的每个变体中，因此不再需要额外的结构体。
### 创建一个新字符串

许多 `Vec<T>` 可用的操作同样适用于 `String`，首先是使用 `new` 函数来创建一个字符串，如下所示。
```rust
    let mut s = String::new();
```

##### 创建一个新的空字符串的示例

这行代码创建了一个名为 `s` 的新空字符串，接下来我们可以向其中加载数据。通常，我们会有一些初始数据需要来开始字符串的内容。为此，我们使用 `to_string` 方法，这个方法在任何实现了 `Display` trait 的类型上都可用，字符串字面量也是如此。下面的示例显示了两种例子。

```rust
    let data = "initial contents";

    let s = data.to_string();

    // 该方法也可以直接在字面量上使用：
    let s = "initial contents".to_string();
```

##### 使用 to_string 方法从字符串字面量创建字符串的示例

这段代码创建了一个包含 `initial contents` 的字符串。

我们还可以使用函数 `String::from` 来从字符串字面值创建一个 `String`。下面的代码等效于之前示例中使用 `to_string` 的代码。

```rust
    let s = String::from("initial contents");
```

##### 使用 String::from 函数从字符串字面值创建字符串的示例

由于字符串用于非常多的用途，我们可以使用许多不同的泛型 API 来处理字符串，这为我们提供了许多选择。有些可能显得冗余，但它们各有其用！在这种情况下，`String::from` 和 `to_string` 功能相同，所以选择哪个只是风格问题。

请记住，字符串是 UTF-8 编码的，因此我们可以在其中包含任何正确编码的数据，如下所示。

```rust
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
```

##### 在字符串中存储不同语言的问候语的示例

所有这些都是合法的 `String` 值。
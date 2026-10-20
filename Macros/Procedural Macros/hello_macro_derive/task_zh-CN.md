### 第2步：定义过程宏

下一步是定义过程宏。在撰写本文时，过程宏需要位于它们自己的crate中。最终，这一限制可能会被取消。用于组织crate和宏crate的惯例如下：对于一个名为`foo`的crate，自定义派生过程宏crate被称为`foo_derive`。

在这里，我们在`Cargo.toml`中定义了新的crate `hello_macro_derive`。

我们的两个crate紧密相关，所以我们在`hello_macro` crate的目录内创建过程宏crate。如果我们更改`hello_macro`中的trait定义，也必须更改`hello_macro_derive`中的过程宏实现。这两个crate需要单独发布，使用这些crate的程序员需要将它们添加为依赖项并将它们引入范围。或者，我们可以让`hello_macro` crate使用`hello_macro_derive`作为依赖项并重新导出过程宏代码。然而，我们的项目结构使得程序员即使不想要`derive`功能，也可以使用`hello_macro`。

我们需要声明`hello_macro_derive` crate为过程宏crate。稍后您将看到，我们还需要`syn`和`quote` crates的功能，因此我们需要将它们添加为依赖项。在`hello_macro_derive`的_Cargo.toml_文件中添加以下内容：

```toml
    [lib]
    proc-macro = true

    [dependencies]
    syn = "1.0"
    quote = "1.0"
```

要开始定义过程宏，请将下面代码片段中的代码放入`hello_macro_derive` crate的_src/lib.rs_文件中。请注意，此代码在我们为`impl_hello_macro`函数添加定义之前不会编译。

```rust
    extern crate proc_macro;

    use crate::proc_macro::TokenStream;
    use quote::quote;
    use syn;

    #[proc_macro_derive(HelloMacro)]
    pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
        // 构造一个Rust代码的语法树表示
        // 我们可以操作的
        let ast = syn::parse(input).unwrap();

        // 构建trait实现
        impl_hello_macro(&ast)
    }
```

##### 处理Rust代码的大多数过程宏crate所需的代码

注意我们已经将代码分为`hello_macro_derive`函数，其负责解析`TokenStream`和`impl_hello_macro`函数，其负责转换语法树：这使得编写过程宏更加方便。外部函数（在此例中的`hello_macro_derive`）的代码将几乎相同于您看到或创建的每一个过程宏crate。您在内部函数（在此例中的`impl_hello_macro`）的主体中指定的代码将根据您的过程宏的目的而不同。

我们引入了三个新crate：`proc_macro`，[`syn`](https://crates.io/crates/syn)，和[`quote`](https://crates.io/crates/quote)。`proc_macro` crate附带在Rust中，因此我们无需将其添加到_Cargo.toml_中的依赖项中。`proc_macro` crate是编译器的API，允许我们从代码中读取和操作Rust代码。

`syn` crate从字符串中解析Rust代码到一个我们可以执行操作的数据结构中。`quote` crate将`syn`数据结构转换回Rust代码。这些crate使解析我们可能想要处理的任何类型的Rust代码变得更加简单：编写完整的Rust代码解析器并不是一项简单的任务。

当我们的库的用户在某个类型上指定`#[derive(HelloMacro)]`时，将调用`hello_macro_derive`函数。这是可能的，因为我们在这里用`proc_macro_derive`注释了`hello_macro_derive`函数，并指定了名称`HelloMacro`，这与我们的trait名称匹配；这是大多数过程宏遵循的惯例。

`hello_macro_derive`函数首先将`input`从`TokenStream`转换为我们可以解释并对其进行操作的数据结构。这就是`syn`派上用场的地方。`syn`中的`parse`函数接受一个`TokenStream`并返回一个表示已解析Rust代码的`DeriveInput`结构体。下面的示例显示了我们从解析`struct Pancakes;`字符串中得到的`DeriveInput`结构体的相关部分：

```rust
    DeriveInput {
        // --snip--

        ident: Ident {
            ident: "Pancakes",
            span: #0 bytes(95..103)
        },
        data: Struct(
            DataStruct {
                struct_token: Struct,
                fields: Unit,
                semi_token: Some(
                    Semi
                )
            }
        )
    }
```

##### 当解析带有宏的属性的代码时得到的DeriveInput实例

此结构体的字段显示我们已解析的Rust代码是一个标识符为`Pancakes`的单元结构体。此结构体上还有更多字段，用于描述各种Rust代码；请查看[`syn`文档中有关`DeriveInput`的信息](https://docs.rs/syn/0.14.4/syn/struct.DeriveInput.html)以获得更多信息。

稍后我们将定义`impl_hello_macro`函数，我们将在其中构建我们想要包含的新Rust代码。但在此之前，请注意，我们的派生宏的输出也是一个`TokenStream`。返回的`TokenStream`将添加到我们的crate用户编写的代码中，因此当他们编译他们的crate时，他们将获得我们在修改后的`TokenStream`中提供的额外功能。

您可能已经注意到，我们这里调用了`unwrap`，以便在对`syn::parse`函数的调用失败时导致`hello_macro_derive`函数崩溃。我们的过程宏在出错时必须崩溃，因为`proc_macro_derive`函数必须返回`TokenStream`而不是`Result`，以符合过程宏的API。我们简化了此示例，使用`unwrap`；在实际代码中，您应使用`panic!`或`expect`提供有关出了什么问题的更具体的错误消息。

现在我们已经有了将注释的Rust代码从`TokenStream`转化为`DeriveInput`实例的代码，让我们生成在注释类型上实现`HelloMacro` trait的代码，如下代码片段所示。

```rust
    fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
        let name = &ast.ident;
        let gen = quote! {
            impl HelloMacro for #name {
                fn hello_macro() {
                    println!("Hello, Macro! My name is {}", stringify!(#name));
                }
            }
        };
        gen.into()
    }
```

##### 使用解析后的Rust代码实现HelloMacro trait

我们通过使用`ast.ident`获得一个包含注释类型名称（标识符）的`Ident`结构体实例。代码段中带有DeriveInput实例的结构体显示，当我们在带有过程宏的示例代码中运行`impl_hello_macro`函数时，我们得到的`ident`将具有值为`"Pancakes"`的`ident`字段。因此，上述代码段中的`name`变量将包含一个在打印时将是字符串`"Pancakes"`的`Ident`结构体实例，名称为在过程宏示例中的结构体的名称。

`quote!`宏允许我们定义我们想要返回的Rust代码。编译器期望与`quote!`宏执行的直接结果不同的内容，因此我们需要将其转换为`TokenStream`。我们通过调用`into`方法来完成此操作，该方法消耗该中间表示并返回所需的`TokenStream`类型值。

`quote!`宏还提供了一些非常酷的模板机制：我们可以输入`#name`，然后`quote!`会用变量`name`中的值替换它。您甚至可以做一些类似于普通宏的重复操作。请查看[`quote` crate的文档](https://docs.rs/quote)以获得详细的介绍。

我们希望我们的过程宏为用户注释的类型生成我们的`HelloMacro` trait的实现，这可以通过使用`#name`来实现。trait实现有一个函数`hello_macro`，其主体包含我们想要提供的功能：打印`Hello, Macro! My name is`，然后是注释类型的名称。

这里使用的`stringify!`宏是内置在Rust中的。它取一个Rust表达式，如`1 + 2`，并在编译时将表达式变为一个字符串字面量，如`"1 + 2"`。这与`format!`或`println!`宏不同，它们在计算表达式后，将结果转换为一个`String`。有可能`#name`输入可能是需要打印的字面表达式，因此我们使用`stringify!`。使用`stringify!`还通过在编译时将`#name`转换为字符串文字节节省了一次分配。

此时，`cargo build`应该在`hello_macro`和`hello_macro_derive`中都能成功完成。让我们将这些crate与使用过程宏的示例代码连接起来，看一下过程宏的实际效果！在您的_projects_目录中使用`cargo new pancakes`创建一个新的二进制项目。我们需要在`pancakes` crate的_Cargo.toml_中添加`hello_macro`和`hello_macro_derive`作为依赖项。如果您将自己的`hello_macro`和`hello_macro_derive`版本发布到_https://crates.io/_，它们将成为常规依赖项；如果没有，您可以将它们指定为`path`依赖项，如下所示：

```toml
    [dependencies]
    hello_macro = { path = "../hello_macro" }
    hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
```

将示例代码中的过程宏代码放入_src/main.rs_，并运行`cargo run`：它应该打印出`Hello, Macro! My name is Pancakes!`过程宏实现的`HelloMacro` trait已经包含在内，而不需要`pancakes` crate实现它；`#[derive(HelloMacro)]`添加了trait实现。

接下来，让我们探索其他类型的过程宏与自定义派生宏如何不同。
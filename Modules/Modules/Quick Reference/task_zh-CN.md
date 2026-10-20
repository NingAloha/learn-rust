## 模块快速参考

这里介绍了模块、路径、`use`关键字和`pub`关键字在编译器中的工作原理，以及大多数开发者如何组织他们的代码。我们会通过一些例子来演示其中的每条规则，但以后回顾模块的工作方式时，这是一个很好的参考。

- **从 crate 根开始**：在编译 crate 时，编译器首先会在 crate 根文件（通常是用于库 crate 的 _src/lib.rs_ 或用于二进制 crate 的 _src/main.rs_）中查找。
- **声明模块**：在 crate 根文件中，你可以声明一个名为“garden”的新模块，例如使用 `mod garden;`。编译器会在以下位置查找模块内的代码：
  - 内联，紧随 `mod garden` 后面，使用大括号而非分号
  - 文件 _src/garden.rs_
  - 文件 _src/garden/mod.rs_
- **声明子模块**：在任何非 crate 根的文件中，它们作为 crate 的一部分进行编译（例如，_src/garden.rs_），你可以声明子模块（例如，`mod vegetables;`）。编译器将在以父模块命名的目录中查找子模块内的代码：
  - 内联，紧随 `mod vegetables` 后面，使用大括号而非分号
  - 文件 _src/garden/vegetables.rs_
  - 文件 _src/garden/vegetables/mod.rs_
- **模块中代码的路径**：一旦模块作为 crate 的一部分进行编译，则可以通过路径 `crate::garden::vegetables::Asparagus` 从 crate 的其他任何地方引用该模块中的代码（例如，garden vegetables 模块中的 `Asparagus` 类型），只要隐私规则允许。
- **私有与公共**：模块内的代码默认是对其父模块私有的。要使一个模块公开，使用 `pub mod` 进行声明而不是 `mod`。要使一个公共模块内的项也变为公开，使用 `pub` 在其声明之前。
- **`use`关键字**：在作用域内，`use`关键字创建对项目的快捷引用，以减少长路径的重复。在可以引用 `crate::garden::vegetables::Asparagus` 的任何作用域中，你可以使用 `use crate::garden::vegetables::Asparagus;` 创建一个快捷方式，接下来在作用域内只需写 `Asparagus` 即可使用该类型。

这是一个名为 `backyard` 的二进制 crate，它展示了这些规则。crate 的目录同样命名为 `backyard`，包含以下文件和目录：

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

crate 根文件，在此为 _src/main.rs_，包含：

文件名：src/main.rs

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

`pub mod garden;` 意味着编译器会包含它在 _src/garden.rs_ 中找到的代码，如下：

文件名：src/garden.rs

```rust
pub mod vegetables;
```

而 `pub mod vegetables;` 表示 _src/garden/vegetables.rs_ 中的代码也被包含进来：

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

现在让我们深入这些规则的细节，并展示它们的实际应用！
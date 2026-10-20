### 将特征作为参数

现在您已经知道如何定义和实现特征，我们可以探讨如何使用特征来定义接收多种不同类型的函数。

例如，在本节的第二个示例中，我们在`NewsArticle`和`Tweet`类型上实现了`Summary`特征。我们可以定义一个`notify`函数，该函数在其`item`参数上调用`summarize`方法，`item`参数的类型实现了`Summary`特征。为此，我们可以使用`impl Trait`语法，如下所示：

```rust,ignore
pub fn notify(item: &impl Summary) {
    println!("突发新闻！{}", item.summarize());
}
```

我们为`item`参数指定了`impl`关键字和特征名，而不是具体类型。此参数接受任何实现指定特征的类型。在`notify`函数体中，我们可以调用来自`Summary`特征的任何方法，例如`summarize`。我们可以调用`notify`并传入任何`NewsArticle`或`Tweet`的实例。调用该函数时传入任何其他类型（例如`String`或`i32`）的代码将无法编译，因为这些类型没有实现`Summary`。

#### 特征约束语法

`impl Trait`语法适用于简单情况，但实际上是较长形式的语法糖，称为*特征约束*，其形式如下：

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
    println!("突发新闻！{}", item.summarize());
}
```

这种较长的形式等同于上一节中的示例，但更详细。我们在泛型类型参数的声明后用冒号以及尖括号内的特征约束来表示。

`impl Trait`语法简单方便，使得代码在简单情况下更简洁。特征约束语法可以在其他情况下表现出更多的复杂性。例如，我们可以有两个实现`Summary`的参数。使用`impl Trait`语法如下面这样：

```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

如果我们希望该函数允许`item1`和`item2`具有不同的类型，那么使用`impl Trait`是合适的（只要这两种类型都实现了`Summary`）。如果我们想强迫两个参数具有相同的类型，那只能用特征约束来表达，如下所示：

```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

泛型类型`T`作为`item1`和`item2`参数的类型限制了函数，使得作为`item1`和`item2`参数传入的具体值的类型必须相同。

#### 使用`+`语法指定多个特征约束

我们还可以指定多个特征约束。假设我们希望`notify`对`item`使用显示格式化以及`summarize`方法：我们在`notify`定义中指定`item`必须同时实现`Display`和`Summary`。我们可以使用`+`语法：

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```

`+`语法对于泛型类型上的特征约束也是有效的：

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

通过指定两个特征约束，`notify`函数体可以调用`summarize`并使用`{}`格式化`item`。

#### 使用`where`子句更清晰的特征约束

使用过多的特征约束有其弊端。每个泛型都有其特征约束，因此具有多个泛型类型参数的函数可能在函数名和参数列表之间包含大量特征约束信息，使得函数签名难以阅读。出于这个原因，Rust提供了在函数签名之后使用`where`子句指定特征约束的替代语法。因此可以写成这样：

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

我们可以使用`where`子句，如下所示：

```rust,ignore
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

这个函数的签名不那么杂乱：函数名、参数列表和返回类型紧密相连，类似于没有大量特征约束的函数。
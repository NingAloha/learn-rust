### 生命周期：超出作用域

接下来，让我们尝试一个示例，这个示例表明`result`中的引用的生命周期必须是两个参数中较短的一个。我们将`result`变量的声明移到内层作用域之外，但将对`result`变量的赋值保持在包含`string2`的作用域内。然后我们将使用`result`的`println!`放到内层作用域之外，即内层作用域结束之后。以下代码将无法编译。

```rust,ignore,does_not_compile
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

#### 尝试在`string2`超出作用域后使用`result`

当我们尝试编译这段代码时，会收到以下错误：

```console
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here
```

错误显示，为了让`result`在`println!`语句中有效，`string2`需要在外层作用域的结束之前有效。Rust知道这一点是因为我们使用相同的生命周期参数`'a`为函数参数和返回值注释了生命周期。

作为人类，我们可以看到这段代码并理解`string1`比`string2`长，因此`result`将包含对`string1`的引用。因为`string1`尚未超出作用域，因此对`string1`的引用在`println!`语句中仍然有效。但是，编译器无法看到引用在这种情况下是有效的。我们已经告诉Rust，由`longest`函数返回的引用的生命周期与传入引用的生命周期中较短的一个相同。因此，借用检查器不允许前面代码中可能包含无效引用的代码。

尝试设计更多实验，改变传入`longest`函数的引用的值和生命周期，以及返回的引用的使用方式。在编译之前，对实验是否会通过借用检查器提出假设；然后检查你的判断是否正确！
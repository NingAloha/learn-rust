### 默认实现

有时，让一个 trait 的某些或所有方法具有默认行为，而不是要求每个类型都为所有方法实现，是很有用的。然后，当我们对特定类型实现该 trait 时，可以保留或重写每个方法的默认行为。

下面的代码展示了如何为 `Summary` trait 的 `summarize` 方法指定默认的字符串，而不是仅定义方法签名，正如我们在本节的第一个示例中所做的那样。

```rust,noplayground
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(阅读全文...)")
    }
}
```

#### 带有 `summarize` 方法默认实现的 `Summary` trait 的定义

为了使用默认实现对 `NewsArticle` 实例进行概括，而不是定义自定义实现，我们使用 `impl Summary for NewsArticle {}` 来指定一个空的 `impl` 块。

即使我们不再直接在 `NewsArticle` 上定义 `summarize` 方法，我们仍然提供了一个默认实现，并指定 `NewsArticle` 实现了 `Summary` trait。因此，我们仍然可以像这样在 `NewsArticle` 的实例上调用 `summarize` 方法：

```rust,ignore
let article = NewsArticle {
    headline: String::from("企鹅队赢得斯坦利杯冠军！"),
    location: String::from("美国宾夕法尼亚州匹兹堡"),
    author: String::from("冰堡"),
    content: String::from(
        "匹兹堡企鹅队再次成为NHL最好的 \
        冰球队。",
    ),
};

println!("新文章上线！{}", article.summarize());
```

此代码将打印 `新文章上线！(阅读全文...)`。

为 `summarize` 创建默认实现并不需要我们对本节第二个示例中的 `Tweet` 上的 `Summary` 实现进行任何更改。原因是重写默认实现的语法与实现没有默认实现的 trait 方法的语法是相同的。
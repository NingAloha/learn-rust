### Iterator Trait 和 next 方法

所有的迭代器都实现了一个名为 `Iterator` 的特征，该特征在标准库中定义。特征的定义如下所示：

```rust
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

        // 方法的默认实现省略
    }
```

注意这个定义使用了一些新的语法：`type Item` 和 `Self::Item`，它们是在为这个特征定义一个 _关联类型_。我们将在第19章深入讨论关联类型。现在，你只需要知道这段代码表示实现 `Iterator` 特征要求你同时定义一个 `Item` 类型，而这个 `Item` 类型被用作 `next` 方法的返回类型。换句话说，`Item` 类型将是迭代器返回的类型。

`Iterator` 特征只要求实现者定义一个方法：`next` 方法，该方法每次返回迭代器的一个项目，用 `Some` 包裹，当迭代结束时，返回 `None`。

我们可以直接在迭代器上调用 `next` 方法；以下示例演示了从向量创建的迭代器多次调用 `next` 返回的值。

```rust
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
```

##### 在迭代器上调用 next 方法

注意我们需要让 `v1_iter` 是可变的：对迭代器调用 `next` 方法会改变迭代器用来跟踪序列位置的内部状态。换句话说，这段代码 _消耗_，或者说用尽了迭代器。每次调用 `next` 都会消耗掉迭代器中的一个项目。当我们使用 `for` 循环时不需要使 `v1_iter` 是可变的，因为循环对 `v1_iter` 进行了所有权转移，并在幕后使其可变。

另外，请注意我们从调用 `next` 获得的值是向量中值的不可变引用。`iter` 方法会生成一个迭代不可变引用的迭代器。如果我们想创建一个拥有 `v1` 所有权并返回拥有值的迭代器，可以调用 `into_iter` 而不是 `iter`。类似地，如果我们想迭代可变引用，可以调用 `iter_mut` 而不是 `iter`。
## 测试一个函数

这个测试没有测试我们的函数 —— 请让它这样做以便测试通过。然后编写第二个测试函数，命名为 `is_false_when_odd`，用来测试当我们调用 `is_even(5)` 时，是否得到预期的结果。

<div class="hint">
  你可以在传递参数给 <code>assert!</code> 时直接调用一个函数 —— 所以你可以这样做：

```rust
assert!(having_fun());
```

  如果你想检查确实得到 `false`，可以使用 `!` 来取反你正在做的事情的结果，例如：

```rust
assert!(!having_fun());
```
</div>
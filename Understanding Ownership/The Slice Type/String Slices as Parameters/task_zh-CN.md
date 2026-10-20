## 将字符串切片作为参数

知道可以对字面量和 `String` 值进行切片之后，我们可以进一步优化 `first_word`，这体现在它的函数签名上：

```rust
    fn first_word(s: &String) -> &str {
```

更有经验的 Rustacean 会写出下面示例所示的签名，因为这样允许我们在 `String` 值和 `&str` 值上都使用相同的函数。

```rust
    fn first_word(s: &str) -> &str {
```

##### 通过将 s 参数的类型改为字符串切片来改进 first_word 函数

如果我们有一个字符串切片，就可以直接传递它。如果我们有一个 `String`，则可以传递整个 `String` 的切片。定义一个函数，使其接受字符串切片而不是 `String` 的引用，可以让我们的 API 更加通用和有用，而不会丢失任何功能：

```rust
fn main() {
    let my_string = String::from("hello world");

    // first_word 可以用于 `String` 的切片
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word 可以用于字符串字面量的切片
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面量本身就是字符串切片，
    // 所以这样也能工作，不需要使用切片语法！
    let word = first_word(my_string_literal);
}
```
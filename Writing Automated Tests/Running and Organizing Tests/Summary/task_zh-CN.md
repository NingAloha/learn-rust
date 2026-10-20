## 概要

Rust 的测试功能提供了一种指定代码应如何运行的方法，以确保即使在进行更改时，它也能按预期工作。单元测试分别测试库的不同部分，并且可以测试私有实现细节。集成测试则检查库的多个部分是否能够正确协作，并使用库的公共 API 来测试代码，就像外部代码会使用它一样。即使 Rust 的类型系统和所有权规则有助于防止某些类型的错误，测试仍然很重要，以减少与代码预期行为相关的逻辑错误。

_您可以参考 Rust Programming Language Book 中的以下章节：[集成测试中的子模块](https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html#submodules-in-integration-tests)_
# Gan - 干就完了！ / Just do it!

[![Crates.io](https://img.shields.io/crates/v/gan)](https://crates.io/crates/gan)
[![Docs.rs](https://docs.rs/gan/badge.svg)](https://docs.rs/gan)

帮助开发者更果決地处理返回值的实用库。提供符合人体工程学的快捷方法来忽略结果/快速构造默认值。
A pragmatic Rust library that helps developers handle return values decisively. Provides ergonomic methods to ignore results or construct default values with semantic shortcuts.

## 快速入门 Quick Start

在 `Cargo.toml` 添加（Add to your Cargo.toml）：
```toml
[dependencies]
gan = "0.1.0"
```

## 用法示例 Usage Examples


| 操作          | 表达式示例                  | 返回类型            | 示例用途                                     |
|-------------|------------------------|-----------------|------------------------------------------|
| **忽略结果**    | `expr.ignore()`        | `()`            | `format!("{}", 42).ignore();`            |
| **构造Ok单元**  | `expr.ok()`            | `Result<(), E>` | `write!(file, "{}", data)?.ok()`         |
| **构造Ok单元**  | `result.ignore().ok()` | `Result<(), E>` | `write!(file, "{}", data).ignore().ok()` |
| **构造Ok包裹**  | `expr.okay()`          | `Result<T, E>`  | `42.okay()`                              |
| **构造Some值** | `expr.some()`          | `Option<T>`     | `42.some()`                              |
| **构造None值** | `expr.none()`          | `Option<U>`     | ` 42.none()`                             |


## 开源协议 License

MIT License © 2024
允许商业使用/修改/分发，无担保责任
Permits commercial use/modification/distribution with no warranty
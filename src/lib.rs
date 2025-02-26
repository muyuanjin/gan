/*!
# Gan - 干就完了！

`干！` 核心原则是帮开发者更果决地处理返回值。提供了忽略结果/快速构造默认值的快捷方式。  
"Just do it!" philosophy. Provides ergonomic value handling with ignore/ok/some semantics.
## Quick Start

Add to your `Cargo.toml` :
```toml
[dependencies]
gan = "0.1"
```

## Usage Examples

| Operation                | Expression Example     | Return Type     | Example Use Case                         |
|--------------------------|------------------------|-----------------|------------------------------------------|
| **Ignore Result**        | `expr.ignore()`        | `()`            | `format!("{}", 42).ignore();`            |
| **Construct Ok Unit**    | `expr.ok()`            | `Result<(), E>` | `write!(file, "{}", data)?.ok()`         |
| **Construct Ok Unit**    | `result.ignore().ok()` | `Result<(), E>` | `write!(file, "{}", data).ignore().ok()` |
| **Construct Ok Wrapped** | `expr.okay()`          | `Result<T, E>`  | `42.okay()`                              |
| **Construct Some Value** | `expr.some()`          | `Option<T>`     | `42.some()`                              |
| **Construct None Value** | `expr.none()`          | `Option<U>`     | `42.none()`                             |
*/
#![doc(html_root_url = "https://docs.rs/gan/latest")]

/// 核心拓展方法集  
/// Core extension trait
pub trait Gan: Sized {
    /// 直接忽略当前值  
    /// Ignores the value completely (alternative to `let _ = ...`)
    ///
    /// # 示例 / Example
    /// ```rust
    /// //使用前 / Before:  
    /// let _ = format!("{}", 42);
    /// //使用后 / After:
    /// use gan::Gan;
    /// format!("{}", 42).ignore();
    /// ```
    #[inline(always)]
    fn ignore(self) {}

    /// 消耗当前值并返回 Ok  
    /// Consumes the value and returns Ok  
    /// 故意命名为ok，由于 Result 的 ok 方法已经被占用，所以避免被误用，Result 必须通过 .ignore().ok()来显式忽略并返回ok  
    /// Named as ok to avoid conflict with Result's ok method. Result must be explicitly ignored and returned as ok using .ignore().ok()
    ///
    /// # 示例 / Example
    /// ```rust
    /// //使用前 / Before:
    /// use std::io::{Write,Error};
    /// use std::fs::OpenOptions;
    /// fn write_(file: &str, data:&str) -> Result<(), Error> {
    ///     let mut file = OpenOptions::new().append(true).open(file)?;
    ///     write!(file, "{}", data)?;
    ///     Ok(())
    /// }
    /// //使用后 / After:
    /// use gan::Gan;
    /// fn write(file: &str, data:&str) -> Result<(), Error> {
    ///     let mut file = OpenOptions::new().append(true).open(file)?;
    ///     write!(file, "{}", data)?.ok()
    ///     // write!(file, "{}", data).ok()// 无法通过编译，防止错误的省略?
    /// }
    /// ```
    #[inline(always)]
    fn ok<E>(self) -> Result<(), E> {
        Ok(())
    }

    /// 返回原值的 Ok 包装  
    /// Wraps self into Ok variant
    ///
    /// # 示例 / Example  
    /// ```rust
    /// //使用前 / Before:
    /// fn get_data_() -> Result<i32, String> {
    ///    Ok(42)
    /// }
    /// //使用后 / After:
    /// use gan::Gan;
    /// fn get_data() -> Result<i32, String> {
    ///   42.okay()
    /// }
    #[inline(always)]
    fn okay<E>(self) -> Result<Self, E> {
        Ok(self)
    }

    /// 返回 Some 包装  
    /// Wraps into Some
    ///
    /// # 示例 / Example  
    /// ```rust
    /// //使用前 / Before:
    /// let opt: Option<i32> = Some(42);
    /// //使用后 / After:
    /// use gan::Gan;
    /// let opt: Option<i32> = 42.some();
    /// ```
    #[inline(always)]
    fn some(self) -> Option<Self> {
        Some(self)
    }

    /// 构造空 None 值  
    /// Creates None of a type while ignoring self
    ///
    /// # 示例 / Example  
    /// ```rust
    /// //使用前 / Before:  
    /// let opt: Option<i32> = None;
    /// //使用后 / After:
    /// use gan::Gan;
    /// let opt: Option<i32> = 42.none();
    /// ```
    #[inline(always)]
    fn none<E>(self) -> Option<E> {
        None
    }
}

// 为所有类型实现 Gan
impl<T: Sized> Gan for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_basic_operations() {
        // Test ignore
        let _ = 42.ignore();

        // Test ok
        assert_eq!("error".ok::<String>(), Ok(()));

        // Test okay
        assert_eq!(42.okay::<String>(), Ok(42));

        // Test some
        assert_eq!(42.some(), Some(42));

        // Test none
        let opt: Option<i32> = 42.none();
        assert!(opt.is_none());
    }

    #[test]
    fn type_inference_samples() {
        // 类型推断测试
        let _: Option<String> = "test".none();
        let _: Result<i32, String> = 123.okay();
    }

    #[test]
    fn result_usage()-> Result<(), String>  {
        // Result 使用测试
        fn get_result() -> Result<(), String> {
            match "42".parse::<i32>() {
                Ok(_) => write!(std::io::stdout(), "Parsed successfully\n").ignore().ok(),
                // Ok(_) => write!(std::io::stdout(), "Parsed successfully\n").ok(),//应该无法通过编译
                Err(_) => Err("Failed to parse".to_string()),
            }
        }
        get_result()
    }

    #[test]
    fn chaining_usage() {
        // 链式调用测试
        let res = "42"
            .parse::<i32>()
            .and_then(|n| n.checked_add(1).unwrap().okay());

        assert_eq!(res, Ok(43));
    }
}
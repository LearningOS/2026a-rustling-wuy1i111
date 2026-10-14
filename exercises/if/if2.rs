// if2.rs
//
// 第 1 步：让我编译通过！
// 第 2 步：让 bar_for_fuzz 和 default_to_baz 这两个测试通过！
//
// 执行 `rustlings hint if2`，或使用 `hint` watch 子命令来获取提示。


pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz" {
        "bar"
    } else {
        "baz"
    }
}

// 测试部分不需要改动！
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}

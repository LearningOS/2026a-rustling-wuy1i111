// if1.rs
//
// 执行 `rustlings hint if1`，或使用 `hint` watch 子命令来获取提示。


pub fn bigger(a: i32, b: i32) -> i32 {
    // 补全这个函数，让它返回两个数中较大的那个！
    // 不要使用：
    // - 调用其他函数
    // - 额外的变量

    if a > b {
        a
    } else {
        b
    }
}

// 现在先不用管这部分 :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}

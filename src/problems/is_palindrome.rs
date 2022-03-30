// 给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。
//
// 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
//
// 例如，121 是回文，而 123 不是。

use std::ops::Mul;

pub fn is_palindrome(x: i32) -> bool {
    let mut prev:i64 = x as i64;
    let mut cur:i64 = 0;

    while prev > 0 {
        cur = cur * 10 + prev % 10;
        prev /= 10;
    }

    cur == x as i64
}

#[test]
fn test() {
    assert_eq!(is_palindrome(-1), false);
    assert_eq!(is_palindrome(0), true);
    assert_eq!(is_palindrome(1), true);
    assert_eq!(is_palindrome(10), false);
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(1000), false);
    assert_eq!(is_palindrome(2147483647), false);
}

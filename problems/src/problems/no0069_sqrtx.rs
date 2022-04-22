// https://leetcode-cn.com/problems/sqrtx/

use std::cmp::Ordering;

// 给你一个非负整数 x ，计算并返回 x 的 算术平方根 。
// 由于返回类型是整数，结果只保留 整数部分 ，小数部分将被 舍去 。
// 注意：不允许使用任何内置指数函数和算符，例如 pow(x, 0.5) 或者 x ** 0.5 。
struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 1 {
            return x;
        }

        let x = x as i64;
        let mut l = 0;
        let mut r = x / 2;

        while l <= r {
            let mid = (l + r) / 2;

            match (mid * mid).cmp(&x) {
                Ordering::Equal => return mid as i32,
                Ordering::Greater => r = mid - 1,
                Ordering::Less => l = mid + 1,
            }
        }

        (l - 1) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::my_sqrt(0), 0);
    assert_eq!(Solution::my_sqrt(1), 1);
    assert_eq!(Solution::my_sqrt(2), 1);
    assert_eq!(Solution::my_sqrt(3), 1);
    assert_eq!(Solution::my_sqrt(4), 2);
    assert_eq!(Solution::my_sqrt(8), 2);
    assert_eq!(Solution::my_sqrt(9), 3);
    assert_eq!(Solution::my_sqrt((2_i64.pow(31) - 1) as i32), 46340);
}

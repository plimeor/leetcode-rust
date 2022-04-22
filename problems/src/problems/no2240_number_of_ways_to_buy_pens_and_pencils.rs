// https://leetcode-cn.com/problems/number-of-ways-to-buy-pens-and-pencils/
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        let max1 = total / cost1;
        let mut result: i64 = 0;

        for i in 0..max1 + 1 {
            // 最多能买 n 个 cost2
            let n: i64 = ((total - i * cost1) / cost2) as i64;
            // 可以选择买 0, 1, 2, .. n 个 cost2
            // 意味着可能性有 n + 1 个
            result += n + 1
        }

        return result;
    }
}

#[test]
fn test() {
    assert_eq!(Solution::ways_to_buy_pens_pencils(20, 10, 5), 9);
    assert_eq!(Solution::ways_to_buy_pens_pencils(5, 10, 10), 1);
    assert_eq!(Solution::ways_to_buy_pens_pencils(100, 1, 1), 1);
}

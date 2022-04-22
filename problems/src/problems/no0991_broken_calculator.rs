use std::collections::{HashMap, HashSet};
use std::thread::current;

// https://leetcode-cn.com/problems/broken-calculator/
struct Solution;

impl Solution {
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        if start_value >= target {
            // 一直执行减 1 操作
            start_value - target
        } else if target % 2 == 0 {
            // 如果 target 偶数，可以不段缩小 target，直到 start_value > target
            1 + Solution::broken_calc(start_value, target / 2)
        } else {
            // 如果 target 是奇数，则先转换为偶数，再执行上面当逻辑
            1 + Solution::broken_calc(start_value, target + 1)
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::broken_calc(2, 3), 2);
    assert_eq!(Solution::broken_calc(5, 8), 2);
    assert_eq!(Solution::broken_calc(3, 10), 3);
    assert_eq!(Solution::broken_calc(17, 12423783), 6);
}

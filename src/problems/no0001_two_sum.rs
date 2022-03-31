// https://leetcode-cn.com/problems/two-sum/

// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出和为目标值 target 的那两个整数，并返回它们的数组下标。
// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
// 你可以按任意顺序返回答案。
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (idx, value) in nums.iter().enumerate() {
            match map.get(&(target - *value)) {
                Some(&other_idx) => return vec![other_idx, idx as i32],
                None => {
                    map.insert(*value, idx as i32);
                }
            }
        }

        vec![]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), [1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), [0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 3, 1, 1], 2), [2, 3]);
}

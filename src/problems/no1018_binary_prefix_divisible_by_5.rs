// https://leetcode-cn.com/problems/binary-prefix-divisible-by-5/
struct Solution;

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut result: Vec<bool> = vec![false; nums.len()];
        let mut val = 0;
        for i in 0..nums.len() {
            val = val * 2 + nums[i];
            val %= 5;
            if val % 5 == 0 {
                result[i] = true;
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::prefixes_div_by5(vec![1, 1, 1, 0, 1]),
        vec![false, false, false, false, false]
    );

    assert_eq!(
        Solution::prefixes_div_by5(vec![1, 1, 0, 0, 0, 1, 0, 0, 1]),
        vec![false, false, false, false, false, false, false, false, false]
    );
}

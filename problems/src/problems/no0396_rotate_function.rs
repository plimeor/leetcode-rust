// https://leetcode-cn.com/problems/rotate-function/
struct Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let mut f_result = 0;
        nums.iter()
            .enumerate()
            .for_each(|(idx, v)| f_result += (idx as i32) * v);

        let n = nums.len();
        let sum: i32 = nums.iter().sum();
        let mut max = f_result;

        for i in 1..n {
            f_result = f_result + sum - (n as i32) * nums[n - i];
            max = max.max(f_result);
        }

        max
    }
}

#[test]
fn test() {}

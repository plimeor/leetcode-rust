// https://leetcode-cn.com/problems/rotate-array/

struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;
        nums[0..n - k].reverse();
        nums[n - k..n].reverse();
        nums.reverse();
    }
}

#[test]
fn test() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    Solution::rotate(&mut nums, k);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
}

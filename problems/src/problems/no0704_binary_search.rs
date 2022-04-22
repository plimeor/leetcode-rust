// https://leetcode-cn.com/problems/binary-search/

// 给定一个 n 个元素有序的（升序）整型数组 nums 和一个目标值 target  ，写一个函数搜索 nums 中的 target，如果目标值存在返回下标，否则返回 -1。
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        if nums[r] < target || nums[l] > target {
            return -1;
        }

        while l <= r {
            let i = (l + r) / 2;
            let cur = nums[i];

            if cur == target {
                return i as i32;
            } else if cur < target {
                l = i + 1;
            } else if cur > target {
                r = i - 1;
            }

        }

        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    assert_eq!(Solution::search(vec![2, 5], 5), 1);
    assert_eq!(Solution::search(vec![5], -5), -1);
}

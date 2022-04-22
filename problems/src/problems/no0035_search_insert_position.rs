// https://leetcode-cn.com/problems/search-insert-position/

// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
//
// 请必须使用时间复杂度为 O(log n) 的算法。
struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        if nums[l] > target {
            return 0
        }

        if nums[r] < target {
            return r as i32 + 1
        }

        while l <= r {
            let i = (l + r) / 2;
            let v = nums[i];

            if v == target {
                return i as i32;
            } else if v > target {
                r = i - 1
            } else {
                l = i + 1;
            }
        }

        l as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::search_insert(vec![1], 0), 0);
    assert_eq!(Solution::search_insert(vec![1], 2), 1);
    assert_eq!(Solution::search_insert(vec![1, 2], 3), 2);
    assert_eq!(Solution::search_insert(vec![1, 2], 0), 0);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
}

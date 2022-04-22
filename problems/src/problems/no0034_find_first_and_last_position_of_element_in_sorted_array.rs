// https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array/

// 给定一个按照升序排列的整数数组 nums，和一个目标值 target。找出给定目标值在数组中的开始位置和结束位置。
//
// 如果数组中不存在目标值 target，返回 [-1, -1]。
//
// 进阶：
// 你可以设计并实现时间复杂度为 O(log n) 的算法解决此问题吗？
struct Solution;

impl Solution {
    // 二分查找 -> 边界查找
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }

        let mut l = 0;
        let mut r = nums.len() - 1;

        if nums[l] > target || nums[r] < target {
            return vec![-1, -1];
        }

        // 寻找左边界
        // 当找到 target 时，保持 r 指向 target，然后逐步收缩左边界，直到 l == r
        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] >= target {
                // 由于 (l + r) / 2 <  r，不会进入死循环
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        if nums[l] != target {
            return vec![-1, -1];
        }

        let left_edge = l;
        l = left_edge;
        r = nums.len() - 1;

        // 寻找右边界（一定存在）
        // 找到 target 时，保持 l == target，然后持续收缩右边界，直到 l == r
        while l < r {
            let mid = (l + r + 1) / 2;
            // 现在只有大于和等于两种可能
            if nums[mid] == target {
                // 由于 ( l + r + 1) / 2 > l，不会进入死循环
                l = mid;
            } else if nums[mid] > target {
                r = mid - 1;
            }
        }

        vec![left_edge as i32, l as i32]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::search_range(vec![5, 6, 7, 8, 8], 6), vec![1, 1]);

    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
        vec![3, 4]
    );

    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
        vec![-1, -1]
    );

    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 12),
        vec![-1, -1]
    );

    assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
}

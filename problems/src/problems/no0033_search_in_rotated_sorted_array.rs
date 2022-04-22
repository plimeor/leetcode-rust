// https://leetcode-cn.com/problems/search-in-rotated-sorted-array/
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut l = 0;
        let mut r = n - 1;

        if n == 1 {
            return if nums[0] == target { 0 } else { -1 };
        }

        while l <= r {
            let mid = (l + r) / 2;

            if nums[mid] == target {
                return mid as i32;
            }

            // mid 在 K 左边
            if nums[mid] >= nums[0] {
                // target 在 mid 左边
                if target >= nums[0] && target < nums[mid] {
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            } else {
                // target 在 mid 右边
                if target > nums[mid] && target <= nums[n - 1] {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::search(vec![1], 0), -1);
    assert_eq!(Solution::search(vec![3, 1], 1), 1);
    assert_eq!(Solution::search(vec![1, 3], 3), 1);
    assert_eq!(Solution::search(vec![1, 2, 3], 3), 2);
    assert_eq!(Solution::search(vec![3, 1, 2], 3), 0);
    assert_eq!(Solution::search(vec![2, 3, 1], 3), 1);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
}

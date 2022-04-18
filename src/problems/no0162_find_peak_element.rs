// https://leetcode-cn.com/problems/find-peak-element/
struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let mid = (l + r + 1) / 2;
            // mid 位于峰值左边
            let is_left = mid < nums.len() - 1 && nums[mid + 1] > nums[mid];
            // mid 位于峰值右边
            let is_right = mid > 0 && nums[mid - 1] > nums[mid];

            if is_left {
                l = mid + 1
            } else if is_right {
                r = mid - 1
            } else {
                return mid as i32;
            }
        }

        l as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
    assert_eq!(Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);
    assert_eq!(Solution::find_peak_element(vec![1, 2, 3]), 2);
    assert_eq!(Solution::find_peak_element(vec![3, 2, 1]), 0);
    assert_eq!(Solution::find_peak_element(vec![1,2,3,1,2,4,5,1]), 2)
}

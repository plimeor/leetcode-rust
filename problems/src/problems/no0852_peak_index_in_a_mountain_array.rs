// https://leetcode-cn.com/problems/peak-index-in-a-mountain-array/

// 符合下列属性的数组 arr 称为 山脉数组 ：
// arr.length >= 3
// 存在 i（0 < i < arr.length - 1）使得：
// arr[0] < arr[1] < ... arr[i-1] < arr[i]
// arr[i] > arr[i+1] > ... > arr[arr.length - 1]
// 给你由整数组成的山脉数组 arr ，返回任何满足 arr[0] < arr[1] < ... arr[i - 1] < arr[i] > arr[i + 1] > ... > arr[arr.length - 1] 的下标 i 。

struct Solution;
impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = arr.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;

            // mid 位于递增区间
            if arr[mid] < arr[mid + 1] {
                left = mid + 1;
            }

            // mid 位于递减区间
            if arr[mid] > arr[mid + 1] {
                right = mid - 1;
            }

        }

        left as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
}

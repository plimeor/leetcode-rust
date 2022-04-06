// https://leetcode-cn.com/problems/kth-missing-positive-number/
struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut l = 0;
        let mut r = arr.len() - 1;

        // 对条件 key < k 进行右边界查找
        while l < r {
            let mid = (l + r + 1) / 2;
            let key = arr[mid] - (mid + 1) as i32;
            if key < k {
                l = mid;
            } else {
                r = mid - 1;
            }
        }

        // arr[l] 缺失的整数数量
        let missed_count = arr[l] - (l + 1) as i32;

        if missed_count < k {
            // 目标位于右侧
            // 之所以加 k，而不是 k - 1，是为了排除 arr[l]
            arr[l] - missed_count + k
        } else {
            // 目标位于左侧
            // 之所以加 k - 1，而不是 k，是为了包含 arr[l] - missed_count
            arr[l] - missed_count + (k - 1)
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_kth_positive(vec![1], 1), 2);
    assert_eq!(Solution::find_kth_positive(vec![5], 5), 6);
    assert_eq!(Solution::find_kth_positive(vec![2], 1), 1);
    assert_eq!(Solution::find_kth_positive(vec![1, 3], 1), 2);
    assert_eq!(Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
    assert_eq!(Solution::find_kth_positive(vec![1, 2, 3, 4], 2), 6);
}

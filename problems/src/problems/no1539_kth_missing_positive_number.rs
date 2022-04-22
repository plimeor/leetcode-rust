// https://leetcode-cn.com/problems/kth-missing-positive-number/
struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut l = 0;
        let mut r = arr.len() - 1;

        if arr[l] > k {
            return k;
        }

        // 对条件 missed_count < k 进行右边界查找
        while l < r {
            let mid = (l + r + 1) / 2;
            // arr[mid] 所缺失对整数数量
            let missed_count = arr[mid] - (mid + 1) as i32;

            // 由于上面判断过 arr[0] <= k
            // 所以第一个 if 语句一定会被执行
            // （如果没执行，就需要在 while 循环后面打补丁了）
            if missed_count < k {
                l = mid;
            } else {
                r = mid - 1;
            }
        }

        // arr[l] 所缺失的整数数量
        let missed_count = arr[l] - (l + 1) as i32;

        k - missed_count + arr[l]
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

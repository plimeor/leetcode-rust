// https://leetcode-cn.com/problems/squares-of-a-sorted-array/

// 给你一个按 非递减顺序 排序的整数数组 nums，返回 每个数字的平方 组成的新数组，要求也按 非递减顺序 排序。
struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        // 关键思路：非递减数组中，绝对值最大的数，不是第一个，就是最后一个
        let n = nums.len();
        let mut i = 0;
        let mut j = n - 1;
        let mut k = n - 1;
        let mut ans = vec![0; n];

        while i < j {
            if nums[i].abs() >= nums[j].abs() {
                ans[k] = nums[i].pow(2);
                i += 1;
            } else {
                ans[k] = nums[j].pow(2);
                j -= 1;
            }
            k -= 1;
        }

        if k == 0 {
            ans[k] = nums[i].pow(2);
        }

        ans
    }

}

#[test]
fn test() {
    assert_eq!(
        Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
        [0, 1, 9, 16, 100]
    );
    assert_eq!(
        Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
        [4, 9, 9, 49, 121]
    );

    assert_eq!(Solution::sorted_squares(vec![-3, 0, 2]), [0, 4, 9]);
    assert_eq!(
        Solution::sorted_squares(vec![-4, -3, -3, -2, 2]),
        [4, 4, 9, 9, 16]
    );
}

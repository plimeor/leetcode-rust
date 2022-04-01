// https://leetcode-cn.com/problems/move-zeroes/

// 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
//
// 请注意 ，必须在不复制数组的情况下原地对数组进行操作。
struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut zero_count = 0;

        for i in 0..n {
            let val = nums[i];

            if val == 0 {
                zero_count += 1
            } else {
                nums.swap(i - zero_count, i);
            }
        }
    }
}

#[test]
fn test() {
    fn case(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = nums.clone();
        Solution::move_zeroes(&mut ans);
        ans
    }

    assert_eq!(case(vec![0, 1, 0, 3, 12]), vec![1, 3, 12, 0, 0]);
    assert_eq!(case(vec![1, 0, 0, 2, 0, 3]), vec![1, 2, 3, 0, 0, 0]);
    assert_eq!(case(vec![1, 0, 0, 0]), vec![1, 0, 0, 0]);
}

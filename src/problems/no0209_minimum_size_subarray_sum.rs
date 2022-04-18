// https://leetcode-cn.com/problems/minimum-size-subarray-sum/
struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut answer = None;
        let mut l = 0;
        let mut sum = 0;

        for (idx, val) in nums.iter().enumerate() {
            sum += val;
            while l != idx && sum - nums[l] >= target {
                sum -= nums[l];
                l += 1;
            }

            if sum >= target {
                let len = idx - l + 1;
                answer = match answer {
                    None => Some(len),
                    Some(val) => Some(val.min(len)),
                };
            }
        }

        answer.unwrap_or(0) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    assert_eq!(
        Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
        0
    );
}

use std::collections::HashSet;

// https://leetcode-cn.com/problems/intersection-of-two-arrays/
struct Solution;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1
            .into_iter()
            .filter(|x| nums2.contains(x))
            .collect::<HashSet<i32>>()
            .into_iter()
            .collect::<Vec<i32>>()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
        vec![2]
    )
}

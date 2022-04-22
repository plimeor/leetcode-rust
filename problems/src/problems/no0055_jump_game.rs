// https://leetcode-cn.com/problems/jump-game/
struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let end = (nums.len() - 1);
        let mut pos: usize = 0;
        let mut max_pos = nums[0] as usize;
        while pos < max_pos {
            if max_pos >= end {
                return true;
            }
            pos += 1;
            max_pos = max_pos.max(pos + nums[pos] as usize);
        }
        pos == end
    }
}

#[test]
fn test() {}

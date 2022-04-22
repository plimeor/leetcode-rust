// https://leetcode-cn.com/problems/maximum-69-number/
struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {

        num.to_string()
            .replacen("6", "9", 1)
            .parse::<i32>()
            .unwrap()
    }
}

#[test]
fn test() {}

// https://leetcode-cn.com/problems/valid-parentheses/
struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let left_pairs = ['(', '[', '{'];
        let right_pairs = [')', ']', '}'];

        let mut stack: Vec<char> = vec![];

        for i in 0..s.len() {
            let c = s[i];

            if left_pairs.contains(&c) {
                stack.push(c);
            } else {
                let idx = right_pairs.iter().position(|&rc| rc == c).unwrap();
                let left_c = left_pairs[idx];
                let mut v = stack.pop();
                if v.is_none() || v.unwrap() != left_c {
                    return false;
                }
            }
        }

        stack.len() == 0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_valid(String::from("({[)")), false);
    assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    assert_eq!(Solution::is_valid(String::from("([)]")), false);
    assert_eq!(Solution::is_valid(String::from("([)]]]")), false);
}

use std::ops::Index;
use std::str::Chars;

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .map(|split| String::from(split.chars().rev().collect::<String>()))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::reverse_words(String::from("Let's take LeetCode contest")),
        String::from("s'teL ekat edoCteeL tsetnoc")
    );

    assert_eq!(
        Solution::reverse_words(String::from("God Ding")),
        String::from("doG gniD")
    );
}

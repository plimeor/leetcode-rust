// https://leetcode-cn.com/problems/reverse-words-in-a-string/
struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.trim()
            .split(' ')
            .map(|x| x.trim())
            .filter(|&x| x != "")
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::reverse_words(String::from("the sky is blue")),
        String::from("blue is sky the")
    );
    assert_eq!(
        Solution::reverse_words(String::from("  hello world  ")),
        String::from("world hello")
    );
    assert_eq!(
        Solution::reverse_words(String::from("a good   example")),
        String::from("example good a")
    );
}

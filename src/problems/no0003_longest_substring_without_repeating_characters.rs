// https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/

// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
// s 由英文字母、数字、符号和空格组成
struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = 0;
        let mut cache = vec![0; 129];
        let mut ret = 0;

        s.chars().enumerate().for_each(|(idx, char)| {
            l = l.max(cache[char as usize]);
            cache[char as usize] = idx + 1;
            ret = ret.max(idx - l + 1);
        });

        ret as i32
    }
}


#[test]
fn test() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("abcabcbb")),
        3
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("bbbbb")),
        1
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("pwwkew")),
        3
    );

    assert_eq!(
        Solution::length_of_longest_substring(String::from("")),
        0
    );

    assert_eq!(
        Solution::length_of_longest_substring(String::from("abcdabcdeabcdef")),
        6
    );

    assert_eq!(
        Solution::length_of_longest_substring(String::from("aabaab!bb")),
        3
    );

    assert_eq!(
        Solution::length_of_longest_substring(String::from("qrsvbspk")),
        5
    );
}

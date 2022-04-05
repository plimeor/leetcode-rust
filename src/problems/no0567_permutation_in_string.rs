// https://leetcode-cn.com/problems/permutation-in-string/

// 给你两个字符串 s1 和 s2 ，写一个函数来判断 s2 是否包含 s1 的排列。如果是，返回 true ；否则，返回 false 。
// 换句话说，s1 的排列之一是 s2 的 子串 。

use std::collections::HashMap;
use std::io::Read;

struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let mut record = [0; 256];
        s1.bytes().for_each(|ch| record[ch as usize] += 1);

        let s = s2.as_bytes();

        s.iter().enumerate().any(|(idx, &ch)| {
            record[ch as usize] -= 1;

            if idx >= s1.len() {
                record[s[idx - s1.len()] as usize] += 1;
            }

            record == [0; 256]
        })
    }
}

#[test]

fn test() {
    assert_eq!(
        Solution::check_inclusion(String::from("ab"), String::from("eidbaooo")),
        true
    );
    assert_eq!(
        Solution::check_inclusion(String::from("ab"), String::from("eidboaoo")),
        false
    );

    assert_eq!(
        Solution::check_inclusion(String::from("adc"), String::from("dcda")),
        true
    );

    assert_eq!(
        Solution::check_inclusion(String::from("ab"), String::from("ba")),
        true
    );
}

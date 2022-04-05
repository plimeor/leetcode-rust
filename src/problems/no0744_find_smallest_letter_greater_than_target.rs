// https://leetcode-cn.com/problems/find-smallest-letter-greater-than-target/

// 给你一个排序后的字符列表 letters ，列表中只包含小写英文字母。另给出一个目标字母 target，请你寻找在这一有序列表里比目标字母大的最小字母。
// 在比较时，字母是依序循环出现的。举个例子：
// 如果目标字母 target = 'z' 并且字符列表为 letters = ['a', 'b']，则答案返回 'a'
struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let next_target = (target as u8 + 1) as char;
        let idx = match  letters.binary_search(&next_target) {
            Ok(i) => i,
            Err(i) => i
        } % letters.len();
        letters[idx]
    }
}

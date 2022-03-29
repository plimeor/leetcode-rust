use std::collections::HashMap;

// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
// s 由英文字母、数字、符号和空格组成
pub fn longest_substring_without_repeating_characters(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let len = s.len();
    if len <= 1 {
        return len as i32;
    }

    let mut substring_length: usize = 1;

    let mut i = 0;
    let mut j = 0;
    let mut cache: HashMap<char, usize> = HashMap::new();

    while i < len {
        let first_char = s[i];
        cache.insert(first_char, i);

        // 不可能出现更长的子序列了
        if len - i <= substring_length {
            break;
        }

        if j <= i {
            j = i + 1;
        }

        while j < len {
            let cur_char = s[j];
            let last_pos = cache.get(&cur_char);

            match last_pos {
                Some(&pos) => {
                    for idx in i..pos + 1 {
                        cache.remove(&s[idx]);
                    }
                    i = pos;
                    break;
                }
                None => {
                    substring_length = substring_length.max(j - i + 1);
                    cache.insert(cur_char, j);
                    j += 1;
                }
            }
        }

        i += 1;
    }

    substring_length as i32
}

#[test]
fn test() {
    assert_eq!(
        longest_substring_without_repeating_characters(String::from("abcabcbb")),
        3
    );
    assert_eq!(
        longest_substring_without_repeating_characters(String::from("bbbbb")),
        1
    );
    assert_eq!(
        longest_substring_without_repeating_characters(String::from("pwwkew")),
        3
    );

    assert_eq!(
        longest_substring_without_repeating_characters(String::from("")),
        0
    );

    assert_eq!(
        longest_substring_without_repeating_characters(String::from("abcdabcdeabcdef")),
        6
    );

    assert_eq!(
        longest_substring_without_repeating_characters(String::from("aabaab!bb")),
        3
    );

    assert_eq!(
        longest_substring_without_repeating_characters(String::from("qrsvbspk")),
        5
    );
}

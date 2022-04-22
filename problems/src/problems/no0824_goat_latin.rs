// https://leetcode-cn.com/problems/goat-latin/
struct Solution;

use std::ops::Add;

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let is_vowel = |char: char| vowels.contains(&char.to_lowercase().next().unwrap());

        sentence
            .split(" ")
            .enumerate()
            .map(|(idx, word)| {
                let mut word = word.chars().collect::<Vec<_>>();
                if !is_vowel(word[0]) {
                    let first_letter = word.remove(0);
                    word.push(first_letter);
                }
                word.push('m');
                word.push('a');
                word.iter().collect::<String>().add(&"a".repeat(idx + 1))
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::to_goat_latin(String::from("I speak Goat Latin"),),
        String::from("Imaa peaksmaaa oatGmaaaa atinLmaaaaa")
    );

    assert_eq!(
        Solution::to_goat_latin(String::from("The quick brown fox jumped over the lazy dog"),),
        String::from("heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa")
    )
}

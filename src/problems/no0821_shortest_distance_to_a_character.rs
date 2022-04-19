// https://leetcode-cn.com/problems/shortest-distance-to-a-character/
struct Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut answer = vec![100000; n];
        let mut last_idx = None;

        for i in 0..n {
            if s[i] == c {
                last_idx = Some(i)
            }
            if let Some(idx) = last_idx {
                answer[i] = answer[i].min((i as i32 - idx as i32).abs())
            }
        }

        for j in 0..n {
            let i = n - j - 1;
            if s[i] == c {
                last_idx = Some(i)
            }
            if let Some(idx) = last_idx {
                answer[i] = answer[i].min((i as i32 - idx as i32).abs())
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::shortest_to_char(String::from("aaab"), 'b'),
        vec![3, 2, 1, 0]
    );

    assert_eq!(
        Solution::shortest_to_char(String::from("loveleetcode"), 'e'),
        vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
    );
}

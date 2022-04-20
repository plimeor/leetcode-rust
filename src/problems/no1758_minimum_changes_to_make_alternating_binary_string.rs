// https://leetcode-cn.com/problems/minimum-changes-to-make-alternating-binary-string/
struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        // 010101...
        let mut step1 = 0;
        // 101010...
        let mut step2 = 0;

        s.chars().enumerate().for_each(|(idx, c)| {
            let even = (idx % 2) == 0;
            let odd = !even;

            if even && c != '0' || odd && c != '1' {
                step1 += 1;
            }

            if even && c != '1'  || odd && c != '0' {
                step2 += 1;
            }
        });

        step1.min(step2)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_operations(String::from("0100")), 1);
    assert_eq!(Solution::min_operations(String::from("10")), 0);
    assert_eq!(Solution::min_operations(String::from("1111")), 2);
}

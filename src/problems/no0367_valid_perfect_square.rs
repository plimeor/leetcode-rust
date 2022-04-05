// https://leetcode-cn.com/problems/valid-perfect-square/

// 给定一个 正整数 num ，编写一个函数，如果 num 是一个完全平方数，则返回 true ，否则返回 false 。
// 进阶：不要 使用任何内置的库函数，如 sqrt 。
struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        let mut l: i64 = 1;
        let mut r: i64 = num / 2;

        while l <= r {
            let mid = l + (r - l) / 2;
            let val = mid * mid;

            if val == num {
                return true;
            } else if val < num {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        l * l == num
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_perfect_square(1), true);
    assert_eq!(Solution::is_perfect_square(16), true);
    assert_eq!(Solution::is_perfect_square(14), false);
    assert_eq!(
        Solution::is_perfect_square((2_i64.pow(31) - 1) as i32),
        false
    );
}

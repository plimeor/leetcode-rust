// https://leetcode-cn.com/problems/powx-n/
struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn pow(x: f64, n: i64) -> f64 {
            if n == 0 {
                return 1.0;
            }
            if n < 0 {
                return 1.0 / pow(x, -n);
            }
            let val = pow(x, n / 2);
            return if n % 2 == 0 { val * val } else { val * val * x };
        }

        pow(x, n as i64)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::my_pow(2.0000, 10), 1024.0);
    assert_eq!(Solution::my_pow(2.1, 3), 9.261000000000001);
    assert_eq!(Solution::my_pow(2.0000, -2), 0.25);
    assert_eq!(Solution::my_pow(1.0000, -2147483648), 1.0);
    assert_eq!(Solution::my_pow(2.0000, -2147483648), 0.0);
}

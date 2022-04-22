// https://leetcode-cn.com/problems/arranging-coins/
struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut l = 1;
        let mut r = n as i64;

        // 右边界查找
        while l < r {
            let mid = (l + r + 1) / 2;
            let val: i64 = mid * (1 + mid) / 2;

            if val <= n as i64 {
                l = mid;
            } else {
                r = mid - 1;
            }
        }

        l as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::arrange_coins(8), 3);
    assert_eq!(Solution::arrange_coins(5), 2);
    assert_eq!(Solution::arrange_coins((2_i64.pow(31) - 1) as i32), 65535);
}

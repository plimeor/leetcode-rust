// https://leetcode-cn.com/problems/guess-number-higher-or-lower/

/**
 * Forward declaration of guess API
 * @param  num   your gues
 * @return 	     -1 if num is lower than the guess numbe
 *			      1 if num is higher than the guess numbe
 *               otherwise return
 * unsafe fn guess(num: i32) -> i32 {
 */
struct Solution;

unsafe fn guess(num: i32) -> i32 {
    1
}

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut l = 1;
        let mut r = n;

        while l <= r {
            let i = l + (r - l) / 2;
            let val = guess(i);

            if val == 0 {
                return i;
            } else if val > 0 {
                l = i + 1;
            } else {
                r = i - 1;
            }
        }
        -1
    }
}

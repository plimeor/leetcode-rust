// https://leetcode-cn.com/problems/minimum-number-of-operations-to-convert-time/
struct Solution;

use std::str::FromStr;

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let current_minutes = Solution::get_minutes(current);
        let correct_minutes = Solution::get_minutes(correct);

        let mut diff = correct_minutes - current_minutes;
        let mut result = 0;

        [60, 15, 5, 1].iter().for_each(|v| {
            result += diff / v;
            diff %= v;
        });

        result
    }

    fn get_minutes(time: String) -> i32 {
        let nums: Vec<i32> = time
            .split(':')
            .map(|char| i32::from_str(char).unwrap())
            .collect();

        return nums[0] * 60 + nums[1];
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::convert_time(String::from("02:30"), String::from("04:35")),
        3
    );

    assert_eq!(
        Solution::convert_time(String::from("11:00"), String::from("11:01")),
        1
    );

    assert_eq!(
        Solution::convert_time(String::from("10:59"), String::from("11:00")),
        1
    );
}

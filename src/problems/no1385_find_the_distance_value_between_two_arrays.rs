// https://leetcode-cn.com/problems/find-the-distance-value-between-two-arrays/

// 给你两个整数数组 arr1 ， arr2 和一个整数 d ，请你返回两个数组之间的 距离值 。
//
// 「距离值」 定义为符合此距离要求的元素数目：对于元素 arr1[i] ，不存在任何元素 arr2[j] 满足 |arr1[i]-arr2[j]| <= d 。
struct Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let get_diff = |&x: &i32| arr2.iter().map(|val| (x - val).abs()).min().unwrap();

        arr1.iter()
            .map(|x| get_diff(x))
            .filter(|&diff| diff > d)
            .collect::<Vec<i32>>()
            .len() as i32
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2),
        2
    );

    assert_eq!(
        Solution::find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3),
        2
    );
}

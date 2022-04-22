// https://leetcode-cn.com/problems/search-a-2d-matrix/
struct Solution;

impl Solution {
    // 这是一个左边界查找
    fn binary_search(list: &Vec<i32>, target: i32) -> Option<usize> {
        let mut l = 0;
        let mut r = list.len() - 1;

        if list[l] > target {
            return None;
        }

        if list[r] < target {
            return Some(r);
        }

        while l < r {
            let mid = (l + r + 1) / 2;
            let val = list[mid];

            if val == target {
                return Some(mid);
            } else if val < target {
                l = mid
            } else {
                r = mid - 1
            }
        }

        Some(l)
    }

    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let heads: Vec<i32> = matrix.iter().map(|line| line[0]).collect();
        // 先确定目标元素位于第几行
        let row = Solution::binary_search(&heads, target).unwrap_or(0);
        // 再确定目标元素位于第几列
        let col = Solution::binary_search(&matrix[row], target).unwrap_or(0);
        // 比较
        matrix[row][col] == target
    }
}

#[test]
fn test() {
    let vec = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert_eq!(Solution::search_matrix(vec.clone(), -1), false);
    assert_eq!(Solution::search_matrix(vec.clone(), 1), true);
    assert_eq!(Solution::search_matrix(vec.clone(), 10), true);
    assert_eq!(Solution::search_matrix(vec.clone(), 11), true);
    assert_eq!(Solution::search_matrix(vec.clone(), 60), true);
}

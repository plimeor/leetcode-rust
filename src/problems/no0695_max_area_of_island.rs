use std::collections::HashSet;

// https://leetcode-cn.com/problems/max-area-of-island/
struct Solution;

impl Solution {
    fn dfs(grid: &mut Vec<Vec<i32>>, cur_i: usize, cur_j: usize) -> i32 {
        if grid[cur_i][cur_j] != 1 {
            return 0;
        }

        let mut count = 1;
        grid[cur_i][cur_j] = 0;

        if cur_i > 0 {
            count += Solution::dfs(grid, cur_i - 1, cur_j);
        }

        if cur_i < grid.len() - 1 {
            count += Solution::dfs(grid, cur_i + 1, cur_j);
        }

        if cur_j > 0 {
            count += Solution::dfs(grid, cur_i, cur_j - 1);
        }

        if cur_j < grid[0].len() - 1 {
            count += Solution::dfs(grid, cur_i, cur_j + 1)
        }

        count
    }

    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut count = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    count = count.max(Solution::dfs(&mut grid, i, j))
                }
            }
        }

        count
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::max_area_of_island(Vec::from([
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
        ])),
        6
    );

    assert_eq!(
        Solution::max_area_of_island(vec![vec![0, 0, 0, 0, 0, 0, 0, 0]]),
        0
    )
}

use std::collections::HashSet;

// https://leetcode-cn.com/problems/flood-fill/
struct Solution;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let m = image.len();
        let n = image[0].len();
        let old_color = image[sr as usize][sc as usize];
        let mut connected_points = HashSet::new();
        let mut queue: Vec<(usize, usize)> = vec![(sr as usize, sc as usize)];

        while !queue.is_empty() {
            let (x, y) = queue.remove(0);

            connected_points.insert((x, y));

            let offsets: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

            offsets
                .iter()
                .map(|&(offset_x, offset_y)| {
                    (
                        (x as i32 + offset_x).min(m as i32 - 1).max(0) as usize,
                        (y as i32 + offset_y).min(n as i32 - 1).max(0) as usize,
                    )
                })
                .filter(|point| image[point.0][point.1] == old_color)
                .filter(|point| !connected_points.contains(point))
                .for_each(|point| queue.push(point));
        }

        let mut result = image.clone();

        connected_points
            .iter()
            .for_each(|point| result[point.0][point.1] = new_color);

        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
        vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
    );

    assert_eq!(
        Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 2),
        vec![vec![2, 2, 2], vec![2, 2, 2]]
    );

    assert_eq!(Solution::flood_fill(vec![vec![0]], 0, 0, 2), vec![vec![2]]);
}

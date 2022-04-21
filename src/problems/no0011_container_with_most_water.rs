// https://leetcode-cn.com/problems/container-with-most-water/
struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area = 0;

        let mut l = 0;
        let mut r = height.len() - 1;

        while l < r {
            let w = (r - l) as i32;
            let h = height[l].min(height[r]);
            area = area.max(w * h);

            while l < r && height[l] <= h {
                l += 1;
            }

            while r > l && height[r] <= h {
                r += 1;
            }
        }

        area
    }
}

#[test]
fn test() {

}

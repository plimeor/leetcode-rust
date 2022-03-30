struct Solution {
    target: i32,
}

impl Solution {
    pub fn new(target: i32) -> Solution {
        Solution { target }
    }

    fn is_bad_version(&self, v: i32) -> bool {
        v >= self.target
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut l = 1;
        let mut r = n;

        while l < r {
            let i = l + (r - l) / 2;

            if self.is_bad_version(i as i32) {
                r = i;
            } else {
                l = i + 1
            }
        }

        l
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::new(1702766719).first_bad_version(2126753390),
        1702766719
    );

    assert_eq!(Solution::new(4).first_bad_version(5), 4);

    assert_eq!(Solution::new(1).first_bad_version(1), 1);
}

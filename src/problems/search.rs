pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;
    let mut i = r / 2;

    if nums[r] < target || nums[l] > target {
        return -1;
    }

    while l < r {
        let cur = nums[i];

        if cur == target {
            return i as i32;
        }else if cur < target {
            l = i + 1;
        }else if cur > target {
            r = i - 1;
        }

        i = (l + r) / 2;
    }

    if nums[l] == target {
        l as i32
    } else {
        -1
    }
}

#[test]
fn test() {
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    assert_eq!(search(vec![2, 5], 5), 1);
    assert_eq!(search(vec![5], -5), -1);
}

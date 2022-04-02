// https://leetcode-cn.com/problems/reverse-string/

// 编写一个函数，其作用是将输入的字符串反转过来。输入字符串以字符数组 s 的形式给出。
//
// 不要给另外的数组分配额外的空间，你必须原地修改输入数组、使用 O(1) 的额外空间解决这一问题。
struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut i = 0;
        let mut j = s.len() - 1;

        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

#[test]
fn test() {
    fn case(mut input: Vec<char>) {
        let mut output = input.clone();
        Solution::reverse_string(&mut output);
        input.reverse();
        assert_eq!(input, output)
    }

    case(vec!['a']);
    case(vec!['a', 'b']);
    case(vec!['a', 'b', 'c']);
    case(vec!['a', 'b', 'c', 'd']);
}

// 一位老师正在出一场由 n 道判断题构成的考试，每道题的答案为 true （用 'T' 表示）或者 false （用 'F' 表示）。老师想增加学生对自己做出答案的不确定性，方法是 最大化 有 连续相同 结果的题数。（也就是连续出现 true 或者连续出现 false）。
//
// 给你一个字符串 answerKey ，其中 answerKey[i] 是第 i 个问题的正确结果。除此以外，还给你一个整数 k ，表示你能进行以下操作的最多次数：
//
// 每次操作中，将问题的正确答案改为 'T' 或者 'F' （也就是将 answerKey[i] 改为 'T' 或者 'F' ）。
// 请你返回在不超过 k 次操作的情况下，最大 连续 'T' 或者 'F' 的数目
pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    fn calc(str: &String, k: i32, flag: char) -> i32 {
        let str: Vec<char> = str.chars().collect();
        let mut l = 0;
        let mut reverse_count = 0;
        let mut result = 0;

        str.iter().enumerate().for_each(|(idx, ch)| {
            if ch != &flag {
                reverse_count += 1;
            }

            if reverse_count <= k {
                result = result.max(idx - l + 1);
            } else {
                if str[l] != flag {
                    reverse_count -= 1;
                }

                l += 1;
            }
        });

        result as i32
    }


    // 如果能知道出现最多的字符的话，应该只需要计算一次就行了
    calc(&answer_key, k, 'T').max(calc(&answer_key, k, 'F'))
}

#[test]
fn test() {
    assert_eq!(max_consecutive_answers(String::from("TTFF"), 2), 4);
    assert_eq!(max_consecutive_answers(String::from("TFFT"), 1), 3);
    assert_eq!(max_consecutive_answers(String::from("TTFTTFTT"), 1), 5);
}

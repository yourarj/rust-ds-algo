pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(input_strings: Vec<String>) -> String {
        let mut ans = String::new();
        'outer: for (i, c) in input_strings[0].char_indices() {
            for str in &input_strings[1..] {
                if str.len() < i + 1 || &str[i..i + 1] != &input_strings[0][i..i + 1] {
                    break 'outer;
                }
            }
            ans.push(c);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result =
            Solution::longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()]);
        assert_eq!(result, "fl");
    }

    #[test]
    fn it_works_2() {
        let result =
            Solution::longest_common_prefix(vec!["dog".into(), "racecar".into(), "car".into()]);
        assert_eq!(result, "");
    }

    #[test]
    fn it_works_3() {
        let result = Solution::longest_common_prefix(vec![
            "baffle".into(),
            "baffler".into(),
            "baffling".into(),
        ]);
        assert_eq!(result, "baffl");
    }

    #[test]
    fn it_works_4() {
        let result = Solution::longest_common_prefix(vec!["ab".into(), "a".into()]);
        assert_eq!(result, "a");
    }
    #[test]
    fn it_works_5() {
        let result = Solution::longest_common_prefix(vec!["abab".into(), "aba".into(), "".into()]);
        assert_eq!(result, "");
    }
    #[test]

    fn it_works_6() {
        let result = Solution::longest_common_prefix(vec!["".into(), "ab".into(), "a".into()]);
        assert_eq!(result, "");
    }
}

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ans = String::new();
        'outer: for (i, c) in strs[0].char_indices() {
            for str in &strs[1..] {
                if str.len() - 1 == i || &str[i..i + 1] != &strs[0][i..i + 1] {
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
}

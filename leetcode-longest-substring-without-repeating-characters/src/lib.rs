use std::{cmp::max, collections::HashMap};

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left = -1;
        let mut right = 0;
        let mut longest_substring = 0;
        let mut char_map = HashMap::with_capacity(s.len());

        for c in s.chars() {
            if let Some(e) = char_map.insert(c, right) {
                left = max(e, left);
            }
            longest_substring = max(longest_substring, right - left);
            right += 1;
        }

        longest_substring
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_01() {
        let input = "abcabcbb".into();
        let expected_result = 3;
        let actual_result = Solution::length_of_longest_substring(input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_02() {
        let input = "bbbbb".into();
        let expected_result = 1;
        let actual_result = Solution::length_of_longest_substring(input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_03() {
        let input = "pwwkew".into();
        let expected_result = 3;
        let actual_result = Solution::length_of_longest_substring(input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_04() {
        let input = "frcgtbbgdderfgvbbhyujhtv".into();
        let expected_result = 7;
        let actual_result = Solution::length_of_longest_substring(input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_05() {
        let input = "".into();
        let expected_result = 0;
        let actual_result = Solution::length_of_longest_substring(input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_06() {
        let input = " ".into();
        let expected_result = 1;
        let actual_result = Solution::length_of_longest_substring(input);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_07() {
        let input = "abcdef".into();
        let expected_result = 6;
        let actual_result = Solution::length_of_longest_substring(input);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn test_08() {
        let input = "tmmzuxt".into();
        let expected_result = 5;
        let actual_result = Solution::length_of_longest_substring(input);
        assert_eq!(actual_result, expected_result);
    }
}

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let map = num.chars().map(|char| char as u8 - '0' as u8).fold(
            HashMap::new(),
            |mut map, digit| {
                *map.entry(digit).or_insert(0) += 1;
                map
            },
        );

        let mut is_match = true;

        for (index, c) in num.chars().enumerate() {
            let expected_occurrence = c as u8 - '0' as u8;
            if map.get(&(index as u8)).unwrap_or(&0) != &expected_occurrence {
                is_match = false;
                break;
            }
        }

        is_match
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_01() {
        let input = "1210";
        assert_eq!(true, Solution::digit_count(input.into()));
    }
    #[test]
    fn test_02() {
        let input = "030";
        assert_eq!(false, Solution::digit_count(input.into()));
    }
}

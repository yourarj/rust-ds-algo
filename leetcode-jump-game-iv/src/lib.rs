pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let neighborhood_map: HashMap<i32, Vec<i32>> = HashMap::new();

        for
        0
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn test_01() {
        let input = vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404];
        let expected_result = 3;
        let actual_result = Solution::min_jumps(input);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn test_02() {
        let input = vec![7];
        let expected_result = 0;
        let actual_result = Solution::min_jumps(input);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn test_03() {
        let input = vec![7, 6, 9, 6, 9, 6, 9, 7];
        let expected_result = 1;
        let actual_result = Solution::min_jumps(input);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn test_04() {
        let input = vec![7, 6, 9, 6, 2, 7, 9, 6, 9, 8, 8, 3, 2];
        // explanation 7 -> 7 -> 2 -> 2
        let expected_result = 3;
        let actual_result = Solution::min_jumps(input);
        assert_eq!(actual_result, expected_result);
    }
}

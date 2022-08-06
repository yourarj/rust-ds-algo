pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        0.0_f64
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_01() {
        let input_one = vec![1, 3];
        let input_two = vec![2];
        let expected_result = 2.0_f64;
        let actual_result = Solution::find_median_sorted_arrays(input_one, input_two);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_02() {
        let input_one = vec![1, 2];
        let input_two = vec![3, 4];
        let expected_result = 2.0_f64;
        let actual_result = Solution::find_median_sorted_arrays(input_one, input_two);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_03() {
        let input_one = vec![];
        let input_two = vec![3, 4];
        let expected_result = 3.5_f64;
        let actual_result = Solution::find_median_sorted_arrays(input_one, input_two);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_04() {
        let input_one = vec![1, 2];
        let input_two = vec![];
        let expected_result = 1.5_f64;
        let actual_result = Solution::find_median_sorted_arrays(input_one, input_two);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_05() {
        let input_one = vec![];
        let input_two = vec![];
        let expected_result = 0.0_f64;
        let actual_result = Solution::find_median_sorted_arrays(input_one, input_two);
        assert_eq!(actual_result, expected_result);
    }
}

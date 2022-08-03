pub struct Solution;

/**
 * We will treat each digit in desired long number as depth of tree
 * and node will be number plus or minus specified difference
 */
impl Solution {
    pub fn nums_same_consec_diff(expected_length: i32, expected_difference: i32) -> Vec<i32> {
        // build up the base vec 0 to 9
        // depth 1
        let mut result_vec: Vec<i32> = (1..=9).collect();

        // depth 2 to expected_length
        for _ in 0..expected_length - 1 {
            // create a new vec with every new depth
            let mut new_level_vec: Vec<i32> = Vec::new();

            // take build up number as base for new number
            for partial_build_num in result_vec.iter() {
                let possible_nums = if expected_difference == 0 {
                    vec![0]
                } else {
                    vec![-expected_difference, expected_difference]
                };

                // check with adding and subtracting diff
                for branch_num in possible_nums.iter() {
                    let new_suffix_num = (partial_build_num % 10) + branch_num;
                    if new_suffix_num >= 0 && new_suffix_num < 10 {
                        new_level_vec.push(partial_build_num * 10 + new_suffix_num);
                    }
                }
            }
            // replace the old vec with new level 2 nums vec
            result_vec = new_level_vec;
        }
        if expected_length == 1 {
            result_vec.push(0);
        }
        result_vec
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_01() {
        let n = 3;
        let k = 7;
        let result = vec![181, 292, 707, 818, 929];
        assert_eq!(result, Solution::nums_same_consec_diff(n, k));
    }

    #[test]
    fn test_02() {
        let n = 2;
        let k = 1;
        let result = vec![
            10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98,
        ];
        assert_eq!(result, Solution::nums_same_consec_diff(n, k));
    }

    #[test]
    fn test_03() {
        let n = 4;
        let k = 5;
        let result = vec![1616, 2727, 3838, 4949, 5050, 6161, 7272, 8383, 9494];
        assert_eq!(result, Solution::nums_same_consec_diff(n, k));
    }
}

use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        let mut max_sum = i64::MIN;
        let mut max_sum_till_now: i64 = 0;

        let arr_sum: i64 = arr.iter().map(|elem| i64::from(*elem)).sum();

        let arr_len = arr.len();
        for idx in 0..arr_len * min(2, k as usize) {
            max_sum_till_now += arr[idx % arr_len] as i64;

            max_sum_till_now = std::cmp::max(0, max_sum_till_now);
            max_sum = std::cmp::max(max_sum, max_sum_till_now);
        }
        (max(max_sum, arr_sum * i64::from(max(0, k - 2)) + max_sum) % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::k_concatenation_max_sum(vec![1, 2], 3);
        assert_eq!(result, 9);
    }

    #[test]
    fn it_works_2() {
        let result = Solution::k_concatenation_max_sum(vec![1, -2, 1], 5);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works_3() {
        let result = Solution::k_concatenation_max_sum(vec![-1, -2], 7);
        assert_eq!(result, 0);
    }

    #[test]
    fn it_works_4() {
        let result = Solution::k_concatenation_max_sum(
            vec![
                10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000,
            ],
            100000,
        );
        assert_eq!(result, 999999937);
    }

    #[test]
    fn it_works_5() {
        let result = Solution::k_concatenation_max_sum(vec![-10000; 100000], 10000);
        assert_eq!(result, 0);
    }
}

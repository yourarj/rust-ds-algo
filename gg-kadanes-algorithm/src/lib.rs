use std::cmp::max;

pub fn max_sum_sub_array(arr: &[i32]) -> i32 {
    let mut max_sum = i32::MIN;
    let mut max_sum_till_now = 0;

    for num in arr {
        max_sum_till_now += num;
        max_sum = max(max_sum, max_sum_till_now);

        // if you want to return 0 in case of max sum is lower than zero
        // move following if check before comparison for max_sum (the earlier statement)
        max_sum_till_now = max(0, max_sum_till_now);
    }
    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr = [1, 2, 3, -2, 5];
        assert_eq!(9, max_sum_sub_array(&arr));
    }

    #[test]
    fn test_2() {
        let arr = [-1, -2, -3, -4];
        assert_eq!(-1, max_sum_sub_array(&arr));
    }

    #[test]
    fn test_3() {
        let arr = [8, -2, -9, -4, 100];
        assert_eq!(100, max_sum_sub_array(&arr));
    }

    #[test]
    fn test_4() {
        let arr = [8, -2, -9, -4, 100, -4];
        assert_eq!(100, max_sum_sub_array(&arr));
    }

    #[test]
    fn test_5() {
        let arr = [1, -2, 1, 1, -2, 1, 1, -2, 1, 1, -2, 1, 1, -2, 1];
        assert_eq!(2, max_sum_sub_array(&arr));
    }
}

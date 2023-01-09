use std::cmp::{max, min};

pub fn get_min_diff(arr: &mut [i32], n: usize, k: i32) -> i32 {
    // sort the given array
    arr.sort();

    // calculate initial min diff without any modification (current worst case)
    let mut min_diff = arr[n - 1] - arr[0];
    
    // check against each element
    for i in 0..n - 1 {
        let (x, y) = (arr[i], arr[i + 1]);
        let tallest = max(arr[n - 1] - k, x + k);
        let shortest = min(arr[0] + k, y - k);
        min_diff = min(min_diff, tallest - shortest);
    }
    min_diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let k = 2;
        let n = 4;
        let mut arr = [1, 5, 8, 10];
        let result = get_min_diff(&mut arr, n, k);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_02() {
        let k = 3;
        let n = 5;
        let mut arr = [3, 9, 12, 16, 20];
        let result = get_min_diff(&mut arr, n, k);
        assert_eq!(result, 11);
    }
    #[test]
    fn test_03_min_case() {
        let k = 1;
        let n = 1;
        let mut arr = [1];
        let result = get_min_diff(&mut arr, n, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_04_max_case() {
        let k = 10000;
        let n = 100000;
        let mut arr = [100000; 100000];
        let result = get_min_diff(&mut arr, n, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_05() {
        let k = 5;
        let n = 10;
        let mut arr = [2, 6, 3, 4, 7, 2, 10, 3, 2, 1];
        let result = get_min_diff(&mut arr, n, k);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_06() {
        let k = 3;
        let n = 3;
        let mut arr = [1, 3, 6];
        let result = get_min_diff(&mut arr, n, k);
        let expected = 3;
        assert_eq!(result, expected);
    }
}

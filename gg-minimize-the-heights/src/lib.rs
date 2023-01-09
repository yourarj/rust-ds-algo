use std::cmp::{max, min};

pub fn get_min_diff(arr: &mut [i32], n: usize, k: i32) -> i32 {
    // sorting the array
    arr.sort();

    // get min difference
    let mut min_diff = arr[n - 1] - arr[0];

    // finding initial minimum difference
    let mut smallest_tower = arr[0] + k;
    let mut largest_tower = arr[n - 1] - k;

    for num in 0..n - 1 {
        smallest_tower = min(smallest_tower, arr[num + 1] - k);
        largest_tower = max(largest_tower, arr[num] + k);

        if smallest_tower < 0 {
            continue;
        }

        min_diff = min(min_diff, largest_tower - smallest_tower);
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

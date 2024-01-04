pub fn sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }
    quick_sort(arr, 0, (arr.len() - 1) as isize);
}

fn quick_sort(arr: &mut [i32], start: isize, end: isize) {
    if start > end {
        return;
    }

    let pivot = arr[(start + (end - start) / 2) as usize];
    let mut new_start = start;
    let mut new_end = end;

    while new_start <= new_end {
        while arr[new_start as usize] < pivot {
            new_start += 1;
        }

        while arr[new_end as usize] > pivot {
            new_end -= 1;
        }

        if new_start <= new_end {
            arr.swap(new_start as usize, new_end as usize);
            new_start += 1;
            new_end -= 1;
        }
    }

    quick_sort(arr, start, new_end);
    quick_sort(arr, new_start, end);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sort_reversed_array() {
        let mut arr = [5, 4, 3, 2, 1, 0];
        sort(&mut arr);

        assert_eq!(arr[0], 0);
        assert_eq!(arr[3], 3);
        assert_eq!(arr[5], 5);
    }

    #[test]
    fn should_sort_array_with_positive_negative_and_zero_numbers() {
        let mut arr = [5, -4, 3, 2, -1, 0];
        sort(&mut arr);

        assert_eq!(arr[0], -4);
        assert_eq!(arr[1], -1);
        assert_eq!(arr[2], 0);
        assert_eq!(arr[3], 2);
        assert_eq!(arr[4], 3);
        assert_eq!(arr[5], 5);
    }

    #[test]
    fn should_not_touch_empty_array() {
        let mut arr = [];
        sort(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn should_sort_array_with_multiple_duplicates_and_some_uniques() {
        let mut arr = [5, 5, 5, 4, 4, 2, 2, 2, 2, 1, 0];
        sort(&mut arr);

        assert_eq!(arr[0], 0);
        assert_eq!(arr[3], 2);
        assert_eq!(arr[5], 2);
    }
}

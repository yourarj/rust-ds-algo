pub fn sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut arr = [5, 4, 3, 2, 1, 0];
        sort(&mut arr);

        assert_eq!(arr[0], 0);
        assert_eq!(arr[3], 3);
        assert_eq!(arr[5], 5);
    }

    #[test]
    fn sort_negative_numbers() {
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
    fn sort_empty_array() {
        let mut arr = [];
        sort(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn it_works_02() {
        let mut arr = [5, 5, 5, 4, 4, 2, 2, 2, 2, 1, 0];
        sort(&mut arr);

        assert_eq!(arr[0], 0);
        assert_eq!(arr[3], 2);
        assert_eq!(arr[5], 2);
    }
}

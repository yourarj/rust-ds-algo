pub fn merge_sort(arr: &mut [i32], start: usize, end: usize) {
    if start < end {
        let mid = (start + end) / 2;
        merge_sort(arr, start, mid);
        merge_sort(arr, mid + 1, end);
        merge(arr, start, mid, end);
    }
}

fn merge(arr: &mut [i32], start: usize, mid: usize, end: usize) {
    let mut left_half = vec![0; arr[start..=mid].len()];
    let mut right_half = vec![0; arr[mid + 1..=end].len()];

    left_half.copy_from_slice(&arr[start..=mid]);
    right_half.copy_from_slice(&arr[mid + 1..=end]);

    let mut l = 0;
    let mut r = 0;
    let mut n = start;

    while l < left_half.len() && r < right_half.len() {
        if left_half[l] <= right_half[r] {
            arr[n] = left_half[l];
            l += 1;
        } else {
            arr[n] = right_half[r];
            r += 1;
        }
        n += 1;
    }

    while l < left_half.len() {
        arr[n] = left_half[l];
        l += 1;
        n += 1;
    }
    while r < right_half.len() {
        arr[n] = right_half[r];
        r += 1;
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut arr = [5, 4, 3, 2, 1, 0];
        merge_sort(&mut arr, 0, 5);

        println!("arr: {:?}", &arr);
        assert_eq!(arr[0], 0);
        assert_eq!(arr[3], 3);
        assert_eq!(arr[5], 5);
    }

    #[test]
    fn it_works_02() {
        let mut arr = [5, 5, 5, 4, 4, 2, 2, 2, 2, 1, 0];
        let end = arr.len() - 1;
        merge_sort(&mut arr, 0, end);

        println!("arr: {:?}", &arr);
        assert_eq!(arr[0], 0);
        assert_eq!(arr[3], 2);
        assert_eq!(arr[5], 2);
    }
}

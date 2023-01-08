pub fn reverse(arr: &mut [i32], left: usize, right: usize) {
    if left >= right {
        return;
    }
    let temp = arr[left];
    arr[left] = arr[right];
    arr[right] = temp;
    reverse(arr, left + 1, right - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut input = [1, 2, 5, 8, 0, 10];
        let left = 0;
        let right = input.len() - 1;
        reverse(&mut input, left, right);
        assert_eq!(input, [10, 0, 8, 5, 2, 1]);
    }

    #[test]
    fn it_works_01() {
        let mut input = [10];
        let left = 0;
        let right = input.len() - 1;
        reverse(&mut input, left, right);
        assert_eq!(input, [10]);
    }
}

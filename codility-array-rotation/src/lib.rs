pub fn solution(input_array: &mut [i32], rotations: i32) -> &[i32] {
    if input_array.is_empty() {
        return input_array;
    }

    for _counter in 0..rotations {
        let last_element = input_array[input_array.len() - 1];

        for index in (0..input_array.len() - 1).rev() {
            input_array[index + 1] = input_array[index];
        }

        input_array[0] = last_element;
    }

    input_array
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn it_works() {
        assert_eq!(solution(&mut [3, 8, 9, 7, 6], 3), [9, 7, 6, 3, 8]);
        assert_eq!(solution(&mut [1, 2, 3, 4], 4), [1, 2, 3, 4]);
    }
}

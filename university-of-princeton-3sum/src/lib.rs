pub struct ThreeSum;
impl ThreeSum {
    pub fn three_sum(values: &Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted = values.clone();
        sorted.sort();
        let mut found_num_sets = Vec::new();
        for (start, elem) in sorted.iter().enumerate() {
            if start > 0 && sorted[start - 1] == *elem {
                // if current element == the previous one, skip it
                // to avoid duplicate
                continue;
            }

            let mut left_index = start + 1;
            let mut right_index = sorted.len() - 1;

            // perform until left and right are different
            while left_index < right_index {
                match sorted[left_index] + sorted[right_index] {
                    sum if sum == -elem => {
                        found_num_sets.push(vec![*elem, sorted[left_index], sorted[right_index]]);
                        left_index = Self::next_valid_left_index(&sorted, left_index);
                        right_index = Self::next_valid_right_index(&sorted, right_index, start);
                    }

                    // when resultant sum exceeding expectations
                    sum if sum > -elem => {
                        right_index = Self::next_valid_right_index(&sorted, right_index, start);
                    }
                    // when resultant sum lagging expectations
                    _ => left_index = Self::next_valid_left_index(&sorted, left_index),
                }
            }
        }
        found_num_sets
    }

    fn next_valid_left_index(values: &Vec<i32>, current_left_index: usize) -> usize {
        let mut next = current_left_index + 1;
        // to avoid outputting duplicates we keep incrementing our index
        // if we encounter the same element
        while next < values.len() && values[next] == values[next - 1] {
            next += 1;
        }
        next
    }

    fn next_valid_right_index(values: &[i32], current_right_index: usize, start: usize) -> usize {
        let mut next = current_right_index - 1;
        while next > start && values[next] == values[next + 1] {
            // to avoid outputting duplicates we keep decrementing our index
            // if we encounter the same element
            next -= 1;
        }
        next
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = vec![-1, 1, 2, -1, 5, -3];
        let result = ThreeSum::three_sum(&arr);
        println!("result is {:?}", result);

        assert!(result.get(0).eq(&Some(&vec![-3, 1, 2])));
        assert!(result.get(1).eq(&Some(&vec![-1, -1, 2])));
        assert_eq!(result.len(), 2);
    }
}

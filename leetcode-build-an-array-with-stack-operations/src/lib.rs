pub struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut steps = Vec::new();
        if target.is_empty() {
            return steps;
        }
        let mut current_index = 0;
        for num in 1..=n {
            if current_index < target.len() {
                steps.push("Push".into());
                if Some(&num) != target.get(current_index) {
                    steps.push("Pop".into());
                } else {
                    current_index += 1;
                }
            } else {
                break;
            }
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expected_result = vec!["Push", "Push", "Pop", "Push"];
        let result = Solution::build_array(vec![1, 3], 3);
        assert_eq!(expected_result, result);
    }
    #[test]
    fn it_works_2() {
        let expected_result = vec!["Push", "Push"];
        let result = Solution::build_array(vec![1, 2], 4);
        assert_eq!(expected_result, result);
    }
    #[test]
    fn it_works_3() {
        let expected_result = vec!["Push", "Push", "Push"];
        let result = Solution::build_array(vec![1, 2, 3], 3);
        assert_eq!(expected_result, result);
    }
}

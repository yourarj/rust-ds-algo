pub struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut last = 0;
        for num in 1..nums.len() {
            if nums[last] != nums[num] {
                last += 1;
                nums[last] = nums[num];
            }
        }
        (last + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut input = vec![1, 1, 2];
        let result = Solution::remove_duplicates(&mut input);
        assert_eq!(result, 2);
    }
    #[test]
    fn it_works_2() {
        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result = Solution::remove_duplicates(&mut input);
        assert_eq!(result, 5);
    }
    #[test]
    fn it_works_3() {
        let mut input = vec![1, 2, 3];
        let result = Solution::remove_duplicates(&mut input);
        assert_eq!(result, 3);
    }
    #[test]
    fn it_works_4() {
        let mut input = vec![1];
        let result = Solution::remove_duplicates(&mut input);
        assert_eq!(result, 1);
    }
}

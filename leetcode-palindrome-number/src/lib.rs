pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut temp_x = x;
        if x < 0 {
            false
        } else {
            let mut swapped_num = 0;
            while temp_x > 0 {
                let num_unit = temp_x % 10;
                temp_x /= 10;
                swapped_num = swapped_num * 10 + num_unit;
            }
            swapped_num == x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_palindrome(101);
        assert!(result);
    }

    #[test]
    fn it_does_not_work_with_negative() {
        let result = Solution::is_palindrome(-101);
        assert!(!result);
    }

    #[test]
    fn it_does_not_work() {
        let result = Solution::is_palindrome(10);
        assert!(!result);
    }
}

pub struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x: i64 = x as i64;
        let mut r: i64 = x;
        while r * r > x {
            r = (r + x / r) / 2;
        }
        return r as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::my_sqrt(4);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works_2() {
        let result = Solution::my_sqrt(8);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works_3() {
        let result = Solution::my_sqrt(17);
        assert_eq!(result, 4);
    }
}

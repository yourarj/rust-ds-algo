fn main() {
    let _sol = Solution::subtract_product_and_sum(235);
}

struct Solution;

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut sum = 0;
        let mut product = 1;

        let mut n = n;

        while n > 0 {
            let digit = n % 10;
            sum += digit;
            product *= digit;

            n /= 10;
        }
        product - sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_positive() {
        let input = 234;
        assert_eq!(15, Solution::subtract_product_and_sum(input));
    }
}

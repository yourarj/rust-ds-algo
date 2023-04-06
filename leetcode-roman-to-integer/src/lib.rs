pub struct Solution;

enum Roman {
    Z = 0,
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl From<char> for Roman {
    fn from(value: char) -> Self {
        match value {
            'I' => Self::I,
            'V' => Self::V,
            'X' => Self::X,
            'L' => Self::L,
            'C' => Self::C,
            'D' => Self::D,
            'M' => Self::M,
            _ => Self::Z,
        }
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let char_vec: Vec<char> = s.chars().collect();

        let mut num = Roman::from(char_vec[char_vec.len() - 1]) as i32;
        for c in (0..s.len() - 1).rev() {
            let right = Roman::from(char_vec[c + 1]) as i32;
            let current = Roman::from(char_vec[c]) as i32;

            if right > current {
                num -= current;
            } else {
                num += current;
            }
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        let result = Solution::roman_to_int("III".into());
        assert_eq!(result, 3);
    }
    #[test]
    fn test_58() {
        let result = Solution::roman_to_int("LVIII".into());
        assert_eq!(result, 58);
    }
    #[test]
    fn test_1994() {
        let result = Solution::roman_to_int("MCMXCIV".into());
        assert_eq!(result, 1994);
    }
}

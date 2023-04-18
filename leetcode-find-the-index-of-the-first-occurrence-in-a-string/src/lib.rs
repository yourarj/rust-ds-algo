pub struct Solution;
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let (hlen, nlen) = (haystack.len(), needle.len());

        if nlen == 0 {
            return 0;
        } else if hlen < nlen {
            return -1;
        }

        for i in 0..=(hlen - nlen) {
            if &haystack[i..i + nlen] == needle {
                return i as i32;
            }
        }

        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::str_str("sadbutsad".into(), "sad".into());
        assert_eq!(result, 0);
    }

    #[test]
    fn it_works_2() {
        let result = Solution::str_str("leetcode".into(), "leetco".into());
        assert_eq!(result, 0);
    }

    #[test]
    fn it_works_3() {
        let result = Solution::str_str("leet_code".into(), "leet_coder".into());
        assert_eq!(result, -1);
    }

    #[test]
    fn it_works_4_needle_at_end() {
        let result = Solution::str_str("I_am_looking_at_leet_code".into(), "leet_code".into());
        assert_eq!(result, 16);
    }
    #[test]
    fn it_works_5_needle_in_between() {
        let result = Solution::str_str("sometimes_leet_code_looks_fine".into(), "leet_code".into());
        assert_eq!(result, 10);
    }

    #[test]
    fn it_works_6_needle_appears_twice() {
        let result = Solution::str_str(
            "sometimes_leet_code_looks_fine_sometimes_leet_code_is_scary".into(),
            "leet_code".into(),
        );
        assert_eq!(result, 10);
    }

    #[test]
    fn it_works_7_needle_appears_twice() {
        let result = Solution::str_str("mississippi".into(), "issip".into());
        assert_eq!(result, 4);
    }
}

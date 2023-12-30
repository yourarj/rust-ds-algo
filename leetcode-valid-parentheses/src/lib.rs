pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let opening_braces: Vec<char> = vec!['{', '[', '('];
        let mut stack = Vec::with_capacity(s.len());

        for par in s.chars() {
            if opening_braces.contains(&par) {
                stack.push(par);
            } else {
                match stack.last() {
                    Some(&'{') => {
                        if par != '}' {
                            break;
                        }
                        stack.pop();
                    }
                    Some(&'[') => {
                        if par != ']' {
                            break;
                        }
                        stack.pop();
                    }
                    Some(&'(') => {
                        if par != ')' {
                            break;
                        }
                        stack.pop();
                    }
                    _ => {
                        stack.push('$');
                        break;
                    }
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_valid("()".into());
        assert!(result);
    }

    #[test]
    fn it_works_2() {
        let result = Solution::is_valid("()[]{}".into());
        assert!(result);
    }

    #[test]
    fn it_works_3() {
        let result = Solution::is_valid("(]".into());
        assert!(!result);
    }

    #[test]
    fn it_works_4() {
        let result = Solution::is_valid("]".into());
        assert!(!result);
    }
}

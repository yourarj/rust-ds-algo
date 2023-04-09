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
                    a if a == Some(&'{') => {
                        if par != '}' {
                            break;
                        }
                        stack.pop();
                    }
                    a if a == Some(&'[') => {
                        if par != ']' {
                            break;
                        }
                        stack.pop();
                    }
                    a if a == Some(&'(') => {
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
        stack.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_valid("()".into());
        assert_eq!(result, true);
    }

    #[test]
    fn it_works_2() {
        let result = Solution::is_valid("()[]{}".into());
        assert_eq!(result, true);
    }

    #[test]
    fn it_works_3() {
        let result = Solution::is_valid("(]".into());
        assert_eq!(result, false);
    }

    #[test]
    fn it_works_4() {
        let result = Solution::is_valid("]".into());
        assert_eq!(result, false);
    }
}

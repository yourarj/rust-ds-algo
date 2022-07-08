fn main() {
    assert_eq!(
        34575,
        Solution::calculate(String::from("12+12+3+34564+2*8/4-10-10"))
    );
}

struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let mut num = 0;
        let mut last_operand = '+';
        for current_char in (s + ".").chars() {
            if ('0'..='9').contains(&current_char) {
                num = num * 10 + (current_char as i32 - '0' as i32);
            } else {
                match last_operand {
                    '+' => stack.push(num),
                    '-' => stack.push(-num),
                    '*' => *stack.last_mut().unwrap() *= num,
                    '/' => *stack.last_mut().unwrap() /= num,
                    _ => {}
                }
                last_operand = current_char;
                num = 0;
            }
        }
        stack.into_iter().sum()
    }
}

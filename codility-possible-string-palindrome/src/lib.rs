pub fn solution(str: &str) -> String {
    let str_length = str.len();
    let approx_mid = str_length as f32 / 2_f32;

    println!("approx mid: {approx_mid}");

    let mid_of_array = approx_mid;

    let mut str_chars: Vec<char> = str.chars().collect();

    for i in 0..mid_of_array.ceil() as usize {
        let right_index = str_length - 1 - i;
        let left_char = str_chars[i];
        let mut right_char = str_chars[right_index];

        if left_char == right_char || left_char == '?' || right_char == '?' {
            if left_char == '?' {
                if right_char == '?' {
                    right_char = 'a';
                    str_chars[right_index] = 'a';
                }
                str_chars[i] = right_char;
            }

            if right_char == '?' {
                str_chars[right_index] = left_char;
            }
        } else {
            return "NO".to_owned();
        }
    }
    str_chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn it_works() {
        assert_eq!("aaa", solution("?a?"));

        assert_eq!("NO", solution("abac?a?"));

        assert_eq!("aaaa", solution("????"));

        assert_eq!("aaaaaaa", solution("???????"));

        assert_eq!("baab", solution("??ab"));

        assert_eq!("ecfdsaaasdfce", solution("???????asdfce"));
    }
}

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut longest_word = String::new();

        for word in dictionary {
            let is_complete_match = is_substring(&s, &word);

            if is_complete_match && word.len() > longest_word.len()
                || (is_complete_match
                    && word.len() == longest_word.len()
                    && word.cmp(&longest_word).eq(&Ordering::Less))
            {
                longest_word = word;
            }
        }
        longest_word
    }
}

fn is_substring(parent: &String, child: &String) -> bool {
    let mut parent_index = 0;
    let mut child_index = 0;
    let p_chars = parent.chars().collect::<Vec<char>>();
    let c_chars = child.chars().collect::<Vec<char>>();
    let mut is_substring = false;
    loop {
        if parent_index == parent.len() || child_index == child.len() {
            break;
        } else if p_chars.get(parent_index).eq(&c_chars.get(child_index)) {
            parent_index += 1;
            child_index += 1;
        } else {
            parent_index += 1;
        }
    }

    if child_index == child.len() {
        is_substring = true;
    }

    is_substring
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_01() {
        let input_s = String::from("abpcplea");
        let input_dictionary = vec!["ale".into(), "apple".into(), "monkey".into(), "plea".into()];
        let result = Solution::find_longest_word(input_s, input_dictionary);
        assert_eq!(result, "apple");
    }

    #[test]
    fn test_02() {
        let input_s = "abpcplea".into();
        let input_dictionary = vec!["a".into(), "b".into(), "c".into()];
        let result = Solution::find_longest_word(input_s, input_dictionary);
        assert_eq!(result, "a");
    }

    #[test]
    fn test_03() {
        let input_s = "aewfafwafjlwajflwajflwafj".into();
        let input_dictionary = vec![
            "apple".into(),
            "ewaf".into(),
            "awefawfwaf".into(),
            "awef".into(),
            "awefe".into(),
            "ewafeffewafewf".into(),
        ];
        let result = Solution::find_longest_word(input_s, input_dictionary);
        assert_eq!(result, "ewaf");
    }
}

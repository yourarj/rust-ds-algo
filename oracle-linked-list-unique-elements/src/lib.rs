pub fn deduplicate_linked_list(list: Vec<i32>) -> Vec<i32> {
    let mut output = vec![];
    for index in 0..list.len() {
        let add_current = match (
            if index == 0 {
                None
            } else {
                list.get(index - 1)
            },
            list[index],
            list.get(index + 1),
        ) {
            (None, _, None) => true,
            (None, current, Some(next)) => current != *next,
            (Some(last), current, Some(next)) => *last != current && current != *next,
            (Some(last), current, None) => *last != current,
        };

        if add_current {
            output.push(list[index])
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use crate::deduplicate_linked_list;

    #[test]
    fn test_deduplicate_linked_list() {
        assert_eq!(
            vec![2, 4],
            deduplicate_linked_list(vec![1, 1, 1, 2, 3, 3, 4])
        );
    }

    // test 1 -> 1 -> 1 -> 2 -> 3 -> 3 -> 4 -> 4
    #[test]
    fn test_deduplicate_linked_list2() {
        assert_eq!(
            vec![2],
            deduplicate_linked_list(vec![1, 1, 1, 2, 3, 3, 4, 4])
        );
    }

    #[test]
    fn test_remove_no_elements_from_all_unique_linked_list() {
        assert_eq!(vec![1, 2, 3, 4], deduplicate_linked_list(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_do_nothing_on_empty_linked_list() {
        assert!(deduplicate_linked_list(vec![]).is_empty());
    }

    #[test]
    fn test_remove_everything_from_all_duplicated_linked_list() {
        assert!(deduplicate_linked_list(vec![100, 100]).is_empty());
    }
}

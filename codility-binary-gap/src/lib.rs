pub fn solution(mut num: i32) -> i32 {
    let mut max_gap = 0;

    let mut current_gap = 0;
    let mut found_one = false;

    while num > 0 {
        if num & 1 == 1 {
            max_gap = std::cmp::max(current_gap, max_gap);
            current_gap = 0;
            found_one = true;
        } else if found_one {
            current_gap += 1;
        }
        num >>= 1;
    }
    max_gap
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

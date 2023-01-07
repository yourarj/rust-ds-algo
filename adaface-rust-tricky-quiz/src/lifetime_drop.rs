use std::fmt::Display;

pub struct Sample;

impl Display for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("A")
    }
}

impl Drop for Sample {
    fn drop(&mut self) {
        print!("B");
    }
}

pub fn f() -> Sample {
    Sample
}

#[cfg(test)]
mod tests {
    use super::Sample;

    #[test]
    fn tester() {
        let Sample;
        print!("{}", Sample);
        println!();
    }
}

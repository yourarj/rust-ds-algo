trait Sample {
    fn func(self);
}

impl<T> Sample for fn(T) {
    fn func(self) {
        print!("A");
    }
}

impl<T> Sample for fn(&T) {
    fn func(self) {
        print!("B");
    }
}

#[cfg(test)]
mod tests {
    use crate::invocations::Sample;

    #[test]
    fn test_a() {
        fn p(_: u8) {}
        fn q(_: &u8) {}
        let first: fn(_) = p;
        let second: fn(_) = q;
        let third: fn(&_) = q;

        first.func();
        second.func();
        third.func();
        println!();
    }
}

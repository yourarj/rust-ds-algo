trait OrTrait {
    fn foo(self);
}

struct Check;

impl OrTrait for &Check {
    fn foo(self) {
        print!("X");
    }
}
impl OrTrait for &&&Check {
    fn foo(self) {
        print!("Y");
    }
}

#[cfg(test)]
mod tests {
    use super::{Check, OrTrait};

    #[test]
    fn tester() {
        let one = &Check;
        let two = &&Check;
        let three = &&&Check;
        let four = &&&&Check;
        one.foo();
        two.foo();
        three.foo();
        four.foo();
        println!();
    }
}

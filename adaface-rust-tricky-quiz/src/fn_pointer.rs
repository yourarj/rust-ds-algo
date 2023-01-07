pub struct Container {
    pub foo: fn(),
}

impl Container {
    pub fn foo(&self) {
        println!("A");
    }
}

#[cfg(test)]
mod tests {
    use super::Container;

    #[test]
    fn foo_test() {
        let f: fn() = || println!("B");

        let c = Container { foo: f };

        // What will be the output 
        // of following statement
        c.foo();
    }
}

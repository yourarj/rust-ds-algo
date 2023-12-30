pub struct Sample;

pub fn foo<X>(_x: X) {
    match std::mem::size_of::<X>() {
        0 => print!("0"),
        _ => print!("1"),
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::{foo, Sample};

    #[test]
    fn foo_test() {
        let sample = &Sample;
        foo(sample);
        #[allow(noop_method_call)]
        foo(sample.clone());

        let sample2 = &();
        foo(sample2);
        #[allow(clippy::unit_arg)]
        #[allow(clippy::clone_on_copy)]
        foo(sample2.clone());

        let sample3 = Rc::new(());
        foo(Rc::clone(&sample3));
        foo(sample3.clone());

        println!();
    }
}

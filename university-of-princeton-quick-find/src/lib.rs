pub struct QuickFind<'arr> {
    arr: &'arr mut [usize],
}

impl<'arr> QuickFind<'arr> {
    pub fn new(arr: &'arr mut [usize]) -> Self {
        QuickFind { arr }
    }

    pub fn connected(&self, a: usize, b: usize) -> bool {
        self.arr[a] == self.arr[b]
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let num = self.arr[a];
        let b_num = self.arr[b];

        for elem in self.arr.iter_mut() {
            if *elem == num {
                *elem = b_num;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

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
    fn it_works() {
        let mut arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut qf = QuickFind::new(&mut arr);
        qf.union(4, 3);
        qf.union(3, 8);
        qf.union(6, 5);
        qf.union(9, 4);
        qf.union(2, 1);
        qf.union(8, 9);
        qf.union(5, 0);
        qf.union(7, 2);
        qf.union(6, 1);
        qf.union(1, 0);
        qf.union(6, 7);

        assert_eq!(false, qf.connected(4, 6));
        assert_eq!(true, qf.connected(3, 4));
        assert_eq!(true, qf.connected(6, 2));
        assert_eq!(true, qf.connected(6, 6));
    }
}

/// Quick union
/// In this we would assume the index as the number itself and
/// value at the index as the parent of the that number
/// if a number points to itsef means it's root of the tree.

pub struct QuickUnion<'arr> {
    arr: &'arr mut [usize],
}

impl<'arr> QuickUnion<'arr> {
    /// create new instance of QuickUnion
    pub fn new(arr: &'arr mut [usize]) -> Self {
        QuickUnion { arr }
    }

    /// find the root of given num
    fn root(&self, mut num: usize) -> usize {
        while self.arr[num] != num {
            num = self.arr[num];
        }
        num
    }

    /// check if specified numbers are connected
    pub fn connected(&self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let root_a = self.root(a);

        self.arr[b] = root_a;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut qf = QuickUnion::new(&mut arr);
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

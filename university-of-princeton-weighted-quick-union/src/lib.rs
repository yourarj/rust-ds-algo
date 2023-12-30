/// Weighted Quick union
/// In this we would assume the index as the number itself and
/// value at the index as the parent of the that number
/// if a number points to itself means it's root of the tree.
/// In addition to data structures in naive Quick Union we also
/// maintain additional array representing weight of the specific ith node.

pub struct WeightedQuickUnion<const SIZE: usize> {
    arr: [usize; SIZE],
    weights: [usize; SIZE],
}

impl<const SIZE: usize> WeightedQuickUnion<SIZE> {
    /// create new instance of QuickUnion
    pub fn new() -> Self {
        let mut arr = [0; SIZE];

        (0..SIZE).for_each(|index| {
            arr[index] = index;
        });
        Self {
            arr,
            weights: [1; SIZE],
        }
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
        // TODO update code to for weight comparison
        let root_a = self.root(a);
        let root_b = self.root(b);

        if root_a == root_b {
            return;
        }

        if self.weights[root_a] < self.weights[root_b] {
            self.arr[root_a] = root_b;
            self.weights[root_b] += self.weights[root_a];
        } else {
            self.arr[root_b] = root_a;
            self.weights[root_a] += self.weights[root_b];
        }
    }
}

impl<const SIZE: usize> Default for WeightedQuickUnion<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut qf = WeightedQuickUnion::<10>::new();
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

        // the above unions creates two isolated connected structures
        // {3,4,8,9}
        // {0,1,2,5,6,7}
        // no element from set 1 is connected to other one
        // and every element in individual is connected to every other one.

        // set one all must be connected
        assert!(qf.connected(3, 4));
        assert!(qf.connected(3, 8));
        assert!(qf.connected(3, 9));
        assert!(qf.connected(4, 8));
        assert!(qf.connected(4, 9));
        assert!(qf.connected(8, 9));

        // must be connected to self
        assert!(qf.connected(6, 6));

        // set two all mus be connected
        assert!(qf.connected(0, 1));
        assert!(qf.connected(0, 2));
        assert!(qf.connected(0, 5));
        assert!(qf.connected(0, 6));
        assert!(qf.connected(0, 7));
        assert!(qf.connected(1, 2));
        assert!(qf.connected(1, 5));
        assert!(qf.connected(1, 6));
        assert!(qf.connected(1, 7));
        assert!(qf.connected(2, 5));
        assert!(qf.connected(2, 6));
        assert!(qf.connected(2, 7));
        assert!(qf.connected(5, 6));
        assert!(qf.connected(5, 7));
        assert!(qf.connected(6, 7));

        // the two sets must not be connected in any way
        assert!(!qf.connected(3, 0));
        assert!(!qf.connected(3, 1));
        assert!(!qf.connected(3, 2));
        assert!(!qf.connected(3, 5));
        assert!(!qf.connected(3, 6));
        assert!(!qf.connected(3, 7));

        assert!(!qf.connected(4, 0));
        assert!(!qf.connected(4, 1));
        assert!(!qf.connected(4, 2));
        assert!(!qf.connected(4, 5));
        assert!(!qf.connected(4, 6));
        assert!(!qf.connected(4, 7));

        assert!(!qf.connected(8, 0));
        assert!(!qf.connected(8, 1));
        assert!(!qf.connected(8, 2));
        assert!(!qf.connected(8, 5));
        assert!(!qf.connected(8, 6));
        assert!(!qf.connected(8, 7));

        assert!(!qf.connected(9, 0));
        assert!(!qf.connected(9, 1));
        assert!(!qf.connected(9, 2));
        assert!(!qf.connected(9, 5));
        assert!(!qf.connected(9, 6));
        assert!(!qf.connected(9, 7));
    }

    #[test]
    fn it_works_from_lecture() {
        let mut qf = WeightedQuickUnion::<10>::new();
        qf.union(4, 3);
        qf.union(3, 8);
        qf.union(6, 5);
        qf.union(9, 4);
        qf.union(2, 1);
        qf.union(5, 0);
        qf.union(7, 2);
        qf.union(6, 1);
        qf.union(7, 3);

        // the above unions creates single connected structure
        //
        // array should be
        // arr = {6, 2, 6, 4, 6, 6, 6, 2, 4, 4}
        //
        // and weights should be
        // weights = {1, 1, 3, 1, 4, 1, 10, 1, 1, 1}

        // set two all mus be connected
        assert!(qf.connected(0, 0));
        assert!(qf.connected(0, 1));
        assert!(qf.connected(0, 2));
        assert!(qf.connected(0, 3));
        assert!(qf.connected(0, 4));
        assert!(qf.connected(0, 5));
        assert!(qf.connected(0, 6));
        assert!(qf.connected(0, 7));
        assert!(qf.connected(0, 8));
        assert!(qf.connected(0, 9));
    }
}

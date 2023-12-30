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

impl<const SIZE: usize> Default for WeightedQuickUnion<SIZE> {
    fn default() -> Self {
        let mut arr = [0; SIZE];

        (0..SIZE).for_each(|index| {
            arr[index] = index;
        });

        Self {
            arr,
            weights: [1; SIZE],
        }
    }
}

impl<const SIZE: usize> WeightedQuickUnion<SIZE> {
    /// create new instance of QuickUnion
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// find the root of given num
    fn root(&mut self, mut num: usize) -> usize {
        while self.arr[num] != num {
            self.arr[num] = self.arr[self.arr[num]];
            num = self.arr[num];
        }
        num
    }

    pub fn completely_connected(&mut self) -> bool {
        let root = self.root(0);
        self.weights.len() == self.weights[root]
    }

    pub fn union(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }

        // TODO update code to for weight comparison
        let root_a = self.root(a);
        let root_b = self.root(b);

        if self.weights[root_a] < self.weights[root_b] {
            self.arr[root_a] = root_b;
            self.weights[root_b] += self.weights[root_a];
        } else {
            self.arr[root_b] = root_a;
            self.weights[root_a] += self.weights[root_b];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert!(qf.completely_connected());
    }
}

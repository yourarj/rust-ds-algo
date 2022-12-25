mod ds;

use ds::WeightedQuickUnion;

pub struct Social<const SIZE: usize> {
    uf: WeightedQuickUnion<SIZE>,
}

impl<const SIZE: usize> Social<SIZE> {
    /// create new instance of Social
    pub fn new() -> Self {
        Social {
            uf: WeightedQuickUnion::new(),
        }
    }

    /// find the root of given num
    pub fn befriend(&mut self, one: usize, two: usize) -> bool {
        self.uf.union(one, two);
        self.uf.completely_connected()
    }

    pub fn everybody_socialized(&mut self) -> bool {
        self.uf.completely_connected()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_from_lecture() {
        let mut soc = Social::<10>::new();
        soc.befriend(4, 3);
        soc.befriend(3, 8);
        soc.befriend(6, 5);
        soc.befriend(9, 4);
        soc.befriend(2, 1);
        soc.befriend(5, 0);
        soc.befriend(7, 2);
        soc.befriend(6, 1);
        soc.befriend(7, 3);
    }
}

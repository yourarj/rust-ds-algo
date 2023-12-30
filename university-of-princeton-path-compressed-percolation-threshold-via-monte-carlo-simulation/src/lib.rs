use university_of_princeton_path_compressed_weighted_quick_union::WeightedQuickUnionFind;

pub struct Percolation<const SIZE: usize> {
    grid_size: usize,
    total_open_sites: usize,
    //TODO SIZE has to be updated but const generics expression
    // still in beta
    grid: WeightedQuickUnionFind<SIZE>,
    virtual_top: usize,
    virtual_bottom: usize,
    grid_open_state: [[bool; SIZE]; SIZE],
}

impl<const SIZE: usize> Default for Percolation<SIZE> {
    fn default() -> Self {
        let total_grid_elements = SIZE * SIZE;
        Self {
            grid_size: SIZE,
            grid: WeightedQuickUnionFind::new(),
            virtual_top: 0,
            // virtual_top + total_grid_elements + virtual_bottom
            virtual_bottom: total_grid_elements + 1,
            total_open_sites: 0,
            grid_open_state: [[false; SIZE]; SIZE],
        }
    }
}

impl<const SIZE: usize> Percolation<SIZE> {
    // creates n-by-n grid, with all sites initially blocked
    pub fn new() -> Self {
        Self::default()
    }

    // opens the site (row, col) if it is not open already
    pub fn open(&mut self, row: usize, col: usize) {
        self.grid_open_state[row][col] = true;
        self.total_open_sites += 1;

        if row == 0 {
            self.grid.union(
                self.virtual_top,
                Self::translate_to_union_find_index(row, col),
            );
        } else if self.is_valid_site(row.checked_add(1), Some(col)) && self.is_open(row + 1, col) {
            self.grid.union(
                Self::translate_to_union_find_index(row, col),
                Self::translate_to_union_find_index(row + 1, col),
            );
        }

        if row == self.grid_size - 1 {
            self.grid.union(
                self.virtual_top,
                Self::translate_to_union_find_index(row, col),
            );
        } else if self.is_valid_site(row.checked_sub(1), Some(col)) && self.is_open(row - 1, col) {
            self.grid.union(
                Self::translate_to_union_find_index(row, col),
                Self::translate_to_union_find_index(row - 1, col),
            );
        }

        if self.is_valid_site(Some(row), col.checked_sub(1)) && self.is_open(row, col - 1) {
            self.grid.union(
                self.virtual_top,
                Self::translate_to_union_find_index(row, col - 1),
            );
        }

        if self.is_valid_site(Some(row), col.checked_add(1)) && self.is_open(row, col + 1) {
            self.grid.union(
                self.virtual_top,
                Self::translate_to_union_find_index(row, col + 1),
            );
        }
    }

    // is the site (row, col) open?
    pub fn is_open(&self, row: usize, col: usize) -> bool {
        self.grid_open_state[row][col]
    }

    // is the site (row, col) full?
    pub fn is_full(&mut self, row: usize, col: usize) -> bool {
        self.grid.connected(
            self.virtual_top,
            Self::translate_to_union_find_index(row, col),
        )
    }

    // returns the number of open sites
    pub fn number_of_open_sites(&self) -> usize {
        self.total_open_sites
    }

    // does the system percolate?
    pub fn percolates(&mut self) -> bool {
        self.grid.connected(self.virtual_top, self.virtual_bottom)
    }

    /// translate given site (row,col combination) to
    /// relative index of union find data structure
    fn translate_to_union_find_index(row: usize, col: usize) -> usize {
        row * col + 1
    }

    // check if given site is valid
    fn is_valid_site(&self, row: Option<usize>, col: Option<usize>) -> bool {
        if let (Some(row), Some(col)) = (row, col) {
            row < self.grid_size && col < self.grid_size
        } else {
            false
        }
    }

    // test client (optional)
    // public static void main(String[] args)
}

#[cfg(test)]
mod tests {
    use crate::Percolation;

    #[test]
    #[ignore = "generic_const_exprs is still unstable"]
    fn it_works() {
        let mut per = Percolation::<4>::new();
        per.open(0, 0);
        per.open(1, 0);
        per.open(2, 0);
        assert!(!per.percolates());
        per.open(3, 0);
        assert!(per.percolates());
    }

    #[test]
    #[ignore = "generic_const_exprs is still unstable"]
    fn it_works_2() {
        let mut per = Percolation::<4>::new();
        per.open(0, 0);
        per.open(0, 1);
        per.open(0, 2);
        per.open(0, 3);

        per.open(1, 3);
        per.open(2, 3);
        per.open(2, 2);
        per.open(2, 1);
        per.open(2, 0);

        assert!(!per.percolates());
        per.open(3, 0);
        assert!(per.percolates());
    }
}

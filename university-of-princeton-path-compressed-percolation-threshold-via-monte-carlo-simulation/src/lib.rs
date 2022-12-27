use university_of_princeton_path_compressed_weighted_quick_union::WeightedQuickUnionFind;

pub struct Percolation<const SIZE: usize> {
    grid_size: usize,
    total_open_sites: usize,
    grid: WeightedQuickUnionFind<SIZE>,
    virtual_top: usize,
    virtual_bottom: usize,
    grid_open_state: [[bool; SIZE]; SIZE],
}

impl<const SIZE: usize> Percolation<SIZE> {
    // creates n-by-n grid, with all sites initially blocked
    pub fn new() -> Self {
        let total_grid_elements = SIZE * SIZE;
        Percolation {
            grid_size: SIZE,
            grid: WeightedQuickUnionFind::new(),
            virtual_top: 0,
            // virtual_top + total_grid_elements + virtual_bottom
            virtual_bottom: total_grid_elements + 2,
            total_open_sites: 0,
            grid_open_state: [[false; SIZE]; SIZE],
        }
    }

    // opens the site (row, col) if it is not open already
    pub fn open(&mut self, row: usize, col: usize) {
        if row == 0 {
            self.grid.union(
                self.virtual_top,
                Self::translate_to_union_find_index(row, col),
            );
        } else if self.is_valid_site(row + 1, col) && self.is_open(row + 1, col) {
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
        } else if self.is_valid_site(row - 1, col) && self.is_open(row - 1, col) {
            self.grid.union(
                Self::translate_to_union_find_index(row, col),
                Self::translate_to_union_find_index(row - 1, col),
            );
        }

        if self.is_valid_site(row, col - 1) && self.is_open(row, col - 1) {
            self.grid.union(
                self.virtual_top,
                Self::translate_to_union_find_index(row, col - 1),
            );
        }

        if self.is_valid_site(row, col + 1) && self.is_open(row, col + 1) {
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
    fn is_valid_site(&self, row: usize, col: usize) -> bool {
        row < self.grid_size && col < self.grid_size
    }

    // test client (optional)
    // public static void main(String[] args)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}

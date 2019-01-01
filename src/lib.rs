//! Half matrix storage, Row major
//! ```ignore
//!  ABCD
//! A-
//! B--
//! C---
//! D----
//! ```
//! 
//! In memory representation:
//! ```ignore
//! -|--|---|----
//! ```
//! 
//! Indexing starts at 0.
//! ```ignore
//! ABCD
//! 0123
//! ```
//!
//! Row Major means that the first parameter of the methods is the Y axis in the matrix, and the second is the X axis.
//!
//! As shown by the matrix, the row value is required to be bigger or equal to the column value.
//! 
//! ### Example
//! Parameters: (3, 0) = (D, A)
//!
//! ```ignore
//!  ABCD
//! A-
//! B--
//! C---
//! DX---
//! ```
//! 

use hibitset::BitSet;

/// A matrix where only half of it is stored. See the lib documentation for more details.
#[derive(Debug, Clone)]
pub struct HalfMatrix {
    size: u32,
    index_size: u32,
    collision_matrix: BitSet,
}

impl HalfMatrix {
    /// Creates a new half matrix of the given size.
    /// This value represents the maximum number of indices you can have for both sides.
    /// ABCD size = 4.
    pub fn new(size: u32) -> Self {
        if size > 5792 {
            panic!("Size of HalfMatrix too big! Maximum size is 5792 and requested size is {}.", size);
        }
        let index_size = (size * (size + 1)) / 2;
        HalfMatrix {
            size,
            index_size,
            collision_matrix: BitSet::with_capacity(index_size),
        }
    }

    /// Returns the size of one side of the matrix.
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Enables the boolean for the position (row, column).
    pub fn enable(&mut self, row: u32, column: u32) {
        self.collision_matrix.add(self.index_of(row, column));
    }

    /// Disables the boolean for the position (row, column).
    pub fn disable(&mut self, row: u32, column: u32) {
        self.collision_matrix.remove(self.index_of(row, column));
    }

    /// Returns the boolean for the position (row, column).
    pub fn contains(&self, row: u32, column: u32) -> bool {
        self.collision_matrix.contains(self.index_of(row, column))
    }

    /// Gives the internal memory index of the bitset for the given (row, column) values.
    /// The internal indexing starts at 0. This means that index_of(0,0) = 0. That is the position of (A, A).
    /// Row is required to be bigger or equal to column.
    pub(crate) fn index_of(&self, row: u32, column: u32) -> u32 {
        // If you entered the indices in the wrong orther, I'll fix them for you. ;)
        let (a, b) = if row >= column {
            (row, column)
        } else {
            (column, row)
        };

        let row_sum = (a * (a + 1)) / 2;
        let idx = row_sum + b;

        assert!(idx < self.index_size);

        idx
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn index_calculation() {
        // 4x4
        // C,C (2,2)
        // Expected: 5
        let m = HalfMatrix::new(4);
        assert_eq!(m.index_of(2,2), 5);
    }

    #[should_panic]
    #[test]
    fn index_of_fail() {
        // 4x4
        // C,C (2,2)
        // Expected: 5
        let m = HalfMatrix::new(1);
        m.index_of(0, 1);
    }

    #[test]
    fn index_calculation2() {
        // 4x4
        // D,C (3,2)
        // Expected: 9
        let m = HalfMatrix::new(4);
        assert_eq!(m.index_of(3,2), 8);
    }

    #[test]
    fn max_size_okay() {
        let size = 5792;
        HalfMatrix::new(size);
    }

    #[test]
    #[should_panic]
    fn max_size_too_big() {
        let size = 5793;
        HalfMatrix::new(size);
    }

    #[test]
    fn enable() {
        // 3x3
        //
        //   012
        // 
        // 2 oxo
        // 1 xx
        // 0 o

        let mut m = HalfMatrix::new(3);
        m.enable(2, 0);
        m.enable(2, 2);
        m.enable(0, 0);

        assert!(m.contains(2,0));
        assert!(m.contains(2,2));
        assert!(m.contains(0,0));

        assert!(!m.contains(1,0));
        assert!(!m.contains(1,1));
        assert!(!m.contains(2,1));
    }

    #[test]
    fn out_of_range_swap() {
        let m = HalfMatrix::new(3);
        m.contains(0,1);
    }
}

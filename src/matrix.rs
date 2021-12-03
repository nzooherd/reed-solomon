use std::fmt;

pub struct Matrix {
    row: u32,
    column: u32,
    data: Vec<Vec<i8>>,

}

impl Matrix {

    pub fn new(row: u32, column: u32) -> Matrix {
        Matrix {
            row, column, data: vec![vec![i8; column]: row]
        }
    }

    fn times(&self, right: &Matrix) -> Matrix {
            todo!()

    }

    fn subMatrix(&self, rmin: u32, cmin: u32, rmax: u32, cmax: u32) -> Matrix {
            todo!()
    }
}


impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}
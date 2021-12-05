use std::fmt;

pub struct Matrix {
    row: u32,
    column: u32,
    data: Vec<Vec<i8>>,

}

impl Matrix {

    pub fn new(row: u32, column: u32) -> Matrix {
        Matrix {
            row, column, data: vec![vec![0; column as usize]; row as usize]
        }
    }

    pub fn get(&self, row: u32, column: u32) -> i8 {
        if row >= self.row {
            panic!("Row index out of range: {}", row)
        }
        if column >= self.column {
            panic!("Column index out of range: {}", column)
        }
        self.data[row as usize][column as usize]
    }

    pub fn set(&mut self, row: u32, column: u32, value: i8) {
        if row >= self.row {
            panic!("Row index out of range: {}", row)
        }
        if column >= self.column {
            panic!("Column index out of range: {}", column)
        }
        self.data[row as usize][column as usize] = value 
    }

    pub fn times(&self, right: &Matrix) -> Matrix {
        if self.column != right.row {
            panic!("Column on left ({}) is different than rows or right ({})", self.column, right.row)
        }
        let mut result = Matrix::new(self.row, right.column);
        for i in 0..self.row {
            for j in 0..right.column {
                let mut value: i8 = 0;
                for k in 0..right.row {
                    value += self.get(i, k) * right.get(k, j)
                }
                (&mut result).set(i, j, value)
            }
        }

        result

    }


    fn subMatrix(&self, rmin: u32, cmin: u32, rmax: u32, cmax: u32) -> Matrix {
        let mut result = Matrix::new(rmax - rmin, cmax - cmin);
        for i in rmin..rmax {
            for j in cmin..cmax {
                result.set(i - rmin, j - cmin, self.get(i, j))
            }
        }
        result

    }

    pub fn augument(&self, right: &Matrix) -> Matrix {
        if self.row != right.row {
            panic!("Matrices don't have the same number of rows")
        }
        let mut result = Matrix::new(self.row, self.column + right.column);
        for i in 0..self.row {
            for j in 0..result.column {
                if j < self.column {
                    (&mut result).set(i, j, self.get(i, j));
                } else {
                    (&mut result).set(i, j, right.get(i, j - self.column));
                }
            }
        }
        return result
    }

    pub fn invert(&self) -> Matrix {
        todo!()
    }


    fn gaussianElimination() {

    }

}


impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[");
        for i in 0..self.row {
            write!(f, "[");
            for j in 0..self.column {
                write!(f, "{}", self.get(i, j));
                if j != self.column - 1 {
                    write!(f, ", ");
                }
            }
            write!(f, "]");

            if i != self.row - 1 {
                write!(f, ", ");
            }
        }
        write!(f, "]")
    }
}
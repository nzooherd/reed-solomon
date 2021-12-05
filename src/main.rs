mod matrix;
use matrix::Matrix;

fn main() {
    let mut m = Matrix::new(4, 4);
    m.set(0, 1, 1);
    m.set(0, 2, 2);
    m.set(1, 3, 3);
    m.set(3, 2, 4);
    m.set(2, 2, 5);
    println!("{}", m);
}

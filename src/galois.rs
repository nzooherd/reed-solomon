pub struct Galois {
    gfilog_table: Vec<i8>,
    gflog_table: Vec<i8>,
}


impl Galois {
    pub fn new() -> Galois {
        let gfilog_table = Galois::generate_gfilog_table();
        Galois {
            gflog_table: Galois::generate_gflog_table(&gfilog_table),
            gfilog_table: gfilog_table
        }
    }

    fn generate_gfilog_table() -> Vec<i8> {
        let mut gfilog_table = Vec::new();
        let mut value: i16 = 1;
        for i in 1..256 {
            if value >= 0x100 {
                value = value ^ 0x11d;
            }
            gfilog_table.push(value as i8);
            value <<= 1
        }

        gfilog_table
    }


    fn generate_gflog_table(gfilog_table: &Vec<i8>) -> Vec<i8> {
        let mut gflog_table = vec![0 as i8; 256];
        for i in 0..255 {
            let value = gflog_table[i];
            gflog_table[value as usize] = i as i8;
        }
        gflog_table
    }

    pub fn add(a: i8, b: i8) -> i8 {
        a ^ b
    }

    pub fn substract(a: i8, b: i8) -> i8 {
        a ^ b
    }

    pub fn multiply(a: i8, b: i8) -> i8 {
        todo!()
    }

    pub fn divide(a: i8, b: i8) -> i8 {
        todo!()
    }
}

fn main() {
    let g = Galois::new();
    for i in 0..255 {
        println!("{}", g.gfilog_table[i]);
    }
}

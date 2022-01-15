use std::fmt;
use lazy_static::lazy_static;

pub struct Galois {
    gfexp_table: Vec<i8>,
    gflog_table: Vec<i8>,
}

lazy_static! {
    static ref GALOIS: Galois = {
        let galois = Galois::new();
        galois
    };
}

#[inline]
pub fn add(a: i8, b: i8) -> i8 {
    a ^ b
}

#[inline]
pub fn sub(a: i8, b: i8) -> i8 {
    a ^ b
}

#[inline]
pub fn mul(a: i8, b: i8) -> i8 {
    if a == 0 || b == 0 {
        0
    } else {
        let log_a: u8 = GALOIS.log(a as u8);
        let log_b: u8 = GALOIS.log(b as u8);
        let mut sum_log: u16 = (log_a as u16) + (log_b as u16);
        if sum_log > 255 {
            sum_log -= 255;
        }
        return GALOIS.exp(sum_log as u8) as i8;
    }
}

#[inline]
pub fn div(a: i8, b: i8) -> i8 {
    if a == 0 || b == 0 {
        0
    } else {
        let log_a: u8 = GALOIS.log(a as u8);
        let log_b: u8 = GALOIS.log(b as u8);
        let mut sub_log: i16 = (log_a as i16) - (log_b as i16);
        if sub_log < 0 {
            sub_log += 255
        }
        return GALOIS.exp(sub_log as u8) as i8;
    }
}

impl Galois {
    pub fn new() -> Galois {
        let gfexp_table = Galois::generate_exp_table();
        let gflog_table = Galois::generate_log_table(&gfexp_table);
        Galois { gfexp_table, gflog_table }
    }

    fn generate_exp_table() -> Vec<i8> {
        let mut gfexp_table = Vec::new();
        let mut value: i16 = 1;
        for i in 0..256 {
            if value >= 0x100 {
                value = value ^ 0x11d;
            }
            gfexp_table.push(value as i8);
            value <<= 1
        }
        gfexp_table
    }


    fn generate_log_table(gfexp_table: &Vec<i8>) -> Vec<i8> {
        let mut gflog_table = vec![0 as i8; 256];
        gflog_table.push(-1);
        for i in 0..255 {
            let value: usize = (gfexp_table[i] as u8).into();
            gflog_table[value] = i as i8;
        }
        gflog_table
    }

    fn log(&self, alog: u8) -> u8 {
        if alog == 0 {
            panic!("The antilogarithm is 0;");
        } else {
            self.gflog_table[alog as usize] as u8
        }
    }

    fn exp(&self, index: u8) -> u8 {
        return self.gfexp_table[index as usize] as u8
    }

}

impl fmt::Display for Galois{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn mul() {
        assert_eq!(super::mul(3, 4), 12);
        assert_eq!(super::mul(7, 7), 21);
        assert_eq!(super::mul(23, 45), 41)
    }

    #[test]
    fn div() {
        assert_eq!(super::div(12, 4), 3);
        assert_eq!(super::div(21, 7), 7);
        assert_eq!(super::div(41, 45), 23)
    }
}

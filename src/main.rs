#![feature(bigint_helper_methods)]
use std::convert::TryInto;

pub struct UInt256 {
    // little endian
    pub limbs: [u64; 4],
}

impl UInt256 {
    // add two Uint256 together
    pub fn add(&self, other: &Self) -> Self {
        let mut carry = false;
        
        Self {
            limbs: self.limbs.iter().zip(other.limbs.iter()).map(|(a, b)| {
                let (mut out, mut new_carry) = a.overflowing_add(*b);
                if carry {
                    (out, new_carry) = out.overflowing_add(1);
                }
                carry = new_carry;
                out    
        }).collect::<Vec<u64>>()
                   .try_into()
                   .expect("4 elem array into 4 elem vec"),
        }
        // final carry not important because this UInt256 will be used for 254-bit modulus
        // arithmetic
    }

    pub fn add_u64(&mut self, other: u64) {
        let mut n = 0;
        let (out, mut carry) = self.limbs[n].overflowing_add(other);
        self.limbs[0] = out;
        while carry && n < 4 {
            (self.limbs[n+1], carry) = self.limbs[n+1].overflowing_add(1);
            n += 1
        }
        
    }
    
    pub fn mul_step(z: u64, x: u64, y: u64, carry: u64) -> (u64, u64) {
        let mut hi: u64;
        let mut lo: u64;
        let mut carry_flag: bool;

        (lo, hi) = x.widening_mul(y);
        (lo, carry_flag) = lo.overflowing_add(carry);
        (hi, _) = hi.overflowing_add(carry_flag as u64);
        (lo, carry_flag) = lo.overflowing_add(z);
        (hi, _) = hi.overflowing_add(carry_flag as u64);

        (hi, lo)
    }

    pub fn mul_hop(z: u64, x: u64, y: u64) -> (u64, u64) {
        let mut hi: u64;
        let mut lo: u64;
        let carry: bool;

        (lo, hi) = x.widening_mul(y);
        (lo, carry) = lo.overflowing_add(z);
        (hi, _) = hi.carrying_add(0, carry);

        (hi, lo)
    }

    pub fn mul(&self, other: &Self) -> [u64; 8] {
        let x = &self.limbs;
        let y = other.limbs;
        let mut res = [0u64; 8];
        let res1: u64;
        let mut res2: u64;
        let mut res3: u64;
        let mut res4: u64;
        let res5: u64;
        let mut carry: u64;
        let carry4: u64;
        let carry5: u64;
        let carry6: u64;
        
        (res[0], carry) = x[0].widening_mul(y[0]);
        (carry, res1) = Self::mul_hop(carry, x[1], y[0]);
        (carry, res2) = Self::mul_hop(carry, x[2], y[0]);
        (carry4, res3) = Self::mul_hop(carry, x[3], y[0]);
        
        (carry, res[1]) = Self::mul_hop(res1, x[0], y[1]);
        (carry, res2) = Self::mul_step(res2, x[1], y[1], carry);
        (carry, res3) = Self::mul_step(res3, x[2], y[1], carry);
        (carry5, res4) = Self::mul_step(carry4, x[3], y[1], carry);
        
        (carry, res[2]) = Self::mul_hop(res2, x[0], y[2]);
        (carry, res3) = Self::mul_step(res3, x[1], y[2], carry);
        (carry, res4) = Self::mul_step(res4, x[2], y[2], carry);
        (carry6, res5) = Self::mul_step(carry5, x[3], y[2], carry);
        
        (carry, res[3]) = Self::mul_hop(res3, x[0], y[3]);
        (carry, res[4]) = Self::mul_step(res4, x[1], y[3], carry);
        (carry, res[5]) = Self::mul_step(res5, x[2], y[3], carry);
        (res[7], res[6]) = Self::mul_step(carry6, x[3], y[3], carry);
        
        res

    }
    

    /*
    // increment UInt256 by one--handles overflow
    pub fn inc(&mut self) {
        let mut flag = true;
        let mut n = 0;
        while flag {
            (self.limbs[n], flag) = self.limbs[n].overflowing_add(1);
            n += 1;
        }
    }
    */
}

// TODO: explore implementing add assign for UInt256 later

fn main() {
    let _test = UInt256 {
        limbs: [u64::MAX, u64::MAX, u64::MAX, 0]
    };

    let test2 = UInt256 {
        limbs: [u64::MAX, 0, 0, 0]
    };

    let test3 = UInt256 {
        limbs: [u64::MAX, 1, 0, 0]
    };

    //println!("u64 MAX: {:#066b}", u64::MAX);
    //
    //println!("TEST: {:?}", test.limbs);
    println!("TEST2: {:?}", test2.limbs);
    /*
    println!("SUM: {:?}", test.add(&test2).limbs);

    println!("TEST3: {:?}", test3.limbs);
    test3.add_u64(u64::MAX);
    println!("TEST3 + u64 MAX: {:?}", test3.limbs);
    */

    //test2.mul(test3);
    println!("PRODUCT: {:?}", test2.mul(&test3));
}

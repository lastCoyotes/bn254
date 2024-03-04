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

imple AddAssign for UInt256 {
    fn add_assign(&mut self, other: Self) {
        
    }
}

fn main() {
    let mut _test = UInt256 {
        limbs: [u64::MAX, u64::MAX, u64::MAX, 0]
    };

    let mut _test2 = UInt256 {
        limbs: [0, u64::MAX, 0, 0]
    };

    let mut _test3 = UInt256 {
        limbs: [0, 0, 0, 0]
    };

    //println!("u64 MAX: {}", u64::MAX);

    println!("_TEST: {:?}", _test.limbs);
    println!("TEST2: {:?}", _test2.limbs);
    
    println!("SUM: {:?}", _test.add(&_test2).limbs);

    println!("TEST3: {:?}", _test3.limbs);
    _test3.add_u64(u64::MAX);
    println!("TEST3 + u64 MAX: {:?}", _test3.limbs);
}

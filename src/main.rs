pub struct Uint256 {
    // little endian
    limbs: [u64; 4],
}

impl Uint256 {
    // add two Uint256 variables together, term that calls this method is the mutated one
    fn add(&mut self, b: Uint256) {
        let mut flag: bool;
        let mut n = 0;
        while n < 4 {
            (self.limbs[n], flag) = self.limbs[n].overflowing_add(b.limbs[n]);
            while flag {
                (self.limbs[n+1], flag) = self.limbs[n+1].overflowing_add(1);
            }
            n += 1
        }
    }

    // increment uint256 by one--handles overflow
    fn inc(&mut self) {
        let mut flag = true;
        let mut n = 0;
        while flag {
            (self.limbs[n], flag) = self.limbs[n].overflowing_add(1);
            n += 1;
        }
    }

}

fn main() {
    let mut _test = Uint256 {
        limbs: [u64::MAX, u64::MAX, u64::MAX, 0]
    };

    let mut _test2 = Uint256 {
        limbs: [0, u64::MAX, 0, 0]
    };

    println!("u64 MAX: {}", u64::MAX);

    println!("_TEST: {:?}", _test.limbs);
    
    _test.inc();

    println!("_TEST ++: {:?}", _test.limbs);

    println!("Hello, world!");
}

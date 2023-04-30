/*
    HyperLogLog Implementation in Rust
    CSCI 5454 Algorithms Final project
    April 2023
    Authors:
        Zack McKevitt
        Jackson Sippe
*/
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

/*
    64 bit hash function
*/
use fasthash::city;
fn h64(data: &str) -> u64 {
    return city::hash64(data);
}

pub struct HyperLogLog {
    p: u32, // First p bits of hashed value
    m: usize, // Number of registers, calculated from p
    am: f64, // Constant used for estimate
    registers: Vec<u64>,
}

impl HyperLogLog {
    pub fn new(p_var: u32) -> HyperLogLog {
        let m_var = usize::pow(2, p_var);

        // TODO: define a_16, a_32, and a_64 separately?
        // We could also use lookup table based on pre-calculated values
        let am_var = 0.7213 / (1.0 + (1.079/(m_var as f64)));
        HyperLogLog {
            p: p_var,
            m: m_var,
            am: am_var,
            registers: vec![0; m_var],
        }
    }

    /*
        Cardinality Estimate
    */
    fn estimate(&mut self) -> f64 {
        let mut sum = 0.0;
        let mut i = 0;
        while i < self.m-1 {
            sum += 1.0 / (u64::pow(2, self.registers[i].try_into().unwrap()) as f64);
            i += 1;
        }
        return self.am * ((self.m*self.m) as f64) * (1.0/sum);
    }

    /*
        Calculate number of leading 0s in binary representation
    */
    fn leading_zeros(&mut self, w: u64) -> u64{
        let mut num_zeros: u64 = 0;
        let mut mask = u64::pow(2, 63-self.p);
        let mut i = 0;
        while i < 64-self.p {
            if w & mask == 0 {
                num_zeros += 1;
                mask = mask >> 1;
            }
            else {
                break;
            }
            i += 1;
        }
        return num_zeros;
    }

    /*
        Calculate number of registers set to 0
    */
    fn num_zero_registers(&mut self) -> u32 {
        let mut sum = 0;
        for i in &self.registers {
            if *i == 0 {
                sum += 1;
            }
        }
        return sum;
    }

    /*
        Linear Counting cardinality estimate
    */
    fn linear_counting(&mut self, v: u32) -> f64 {
        // Assuming log with base 2
        return (self.m as f64) * ((v as f64)/(self.m as f64)).log(2.0); 
    }

    /*
        Aggregation phase
    */
    pub fn aggregation(&mut self, fname: &str) -> io::Result<()> {
        // Data Stream
        let file = File::open(fname)?;
        let lines = BufReader::new(file).lines();

        // Read lines into memory one by one
        for line in lines {

            let line_str = line?;
            let words = line_str.split(" ");

            for word in words {
                let hash: u64 = h64(word);
                
                // Get first p bits of the hash
                let idx = hash >> (64 - self.p);

                // Get rest of bits
                let w = hash & (u64::pow(2, 64 - self.p) - 1);

                // This is currently preventing us from using u64 hash
                self.registers[idx as usize] = std::cmp::max(self.registers[idx as usize], self.leading_zeros(w));
            }
        }

        Ok(())
    }

    /*
        Compute Result
    */
    pub fn result_computation(&mut self) -> f64 {
        let mut e = self.estimate();
        if e < 0.4 * (self.m as f64) {
            // Number of zero registers
            let v = self.num_zero_registers();
            if v != 0 {
                e = self.linear_counting(v);
            }
        }
        else if e < (1.0/30.0) * (u64::pow(2, 32) as f64){
            // Do Nothing 
        }
        else {
            // Assuming log base 2
            e = (-1.0 * (u64::pow(2,32) as f64) * (1.0 - e/(u64::pow(2,32) as f64))).log(2.0);
        }
        return e;
    }
}

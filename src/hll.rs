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
use fasthash::city;
use sha256::{digest};
use byteorder::{ByteOrder, BigEndian};

/*
    64 bit hash function
*/
fn h64(data: &str) -> u64 {
    return city::hash64(data);
}

fn _h_sha(data: &str) -> u64 {
    let full_hash = digest(data);
    let binding = hex::decode(full_hash[0..16].as_bytes()).expect("Decode failed");
    let res = BigEndian::read_u64(binding.as_slice());
    return res
}

pub struct HyperLogLog {
    p: u32, // First p bits of hashed value
    m: usize, // Number of registers, calculated from p
    am: f64, // Constant used for estimate
    pub registers: Vec<u64>,
}

impl HyperLogLog {
    pub fn new(p_var: u32) -> HyperLogLog {
        let m_var = usize::pow(2, p_var);

        let am_var = match m_var {
            16 => 0.673,
            32 => 0.697,
            64 => 0.709,
            _ => 0.7213 / (1.0 + (1.079/(m_var as f64)))
        };

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
        let z: f64 = 1.0 / self.registers.iter().map(|r| 2.0_f64.powf(-(*r as i64) as f64)).sum::<f64>();
        return self.am * ((self.m*self.m) as f64) * z
    }

    /*
        Calculate number of leading 0s in binary representation
    */
    fn leading_zeros(&mut self, w: u64) -> u64{
        let mut num_zeros: u64 = 0;
        let mut mask = u64::pow(2, 64-self.p);
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
                // Create masks for index and leading zeroes calc
                let mask: u64 = (1<<(64-self.p)) - 1;
                let idx_mask = 0xffffffffffffffff ^ mask;

                // Hash the given input
                let hash: u64 = h64(word);

                // Get first P bits of the hash to use as index
                let idx = ((hash & idx_mask) >> (64-self.p)) as usize;

                // Get rest of bits and calculate leading zeros
                let w = hash & mask;
                let leading_zeros = self.leading_zeros(w);

                // This is currently preventing us from using u64 hash
                self.registers[idx] = std::cmp::max(self.registers[idx], leading_zeros);
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
            e = (-1.0 * (u64::pow(2,32) as f64) * (1.0 - e/(u64::pow(2,32) as f64))).ln();
        }
        return e;
    }
}

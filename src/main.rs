/*
    HyperLogLog Implementation in Rust
    CSCI 5454 Algorithms Final Project
    April 2023
    Authors:
        Zack McKevitt
        Jackson Sippe
*/

/* 
    First P bits of hashed value
    Technically only need 8 bits for P,
    but u32 avoids casting a bunch
*/
const P: u32 = 6;

/*
    Number of registers
*/
const M: usize = usize::pow(2, P);

/*
    Constant used for estimate
    TODO: define a_16, a_32, and a_64 separately?
*/
const AM: f64 = 0.7213 / (1.0 + (1.079/(M as f64)));

/*
    64 bit hash function
*/
use fasthash::city;
fn _h64(data: &str) -> u64 {
    return city::hash64(data);
}

/*
    32 bit hash function
    Currently cant convert u64->usize
    as needed for array indexing
    WANT to use 64, HLL++
*/
fn h32(data: &str) -> u32 {
    return city::hash32(data);
}

/*
    Cardinality Estimate
*/
fn estimate(registers: &mut [u32; M]) -> f64 {
    let mut sum = 0.0;
    let mut i = 0;
    while i < M-1 {
        sum += 1.0 / (u32::pow(2, registers[i]) as f64);
        i += 1;
    }
    return AM * ((M*M) as f64) * (1.0/sum);
}

/*
    Calculate number of leading 0s in binary representation
    TODO: change to 64 bit input
*/
fn leading_zeros(w: u32) -> u32{
    let mut num_zeros: u32 = 0;
    let mut mask = u32::pow(2, 31-P);
    let mut i = 0;
    while i < 32-P {
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
fn num_zero_registers(registers: &[u32;M]) -> u32 {
    let mut sum = 0;
    for i in registers {
        if *i == 0 {
            sum += 1;
        }
        println!("{}", i);
    }
    return sum;
}

/*
    Linear Countring cardinality estimate
*/
fn linear_counting(v: u32) -> f64 {
    // Assuming log with base 2
    return (M as f64) * ((v as f64)/(M as f64)).log(2.0); 
}

/*
    Initialization phase
    Define constants and allocate registers
    TODO:
        Maybe change the way this is done?
        Can we keep a global mutable pointer that 
            points to dynamically allocated
            register array?
*/
fn init_registers() -> [u32; M] {
    let registers: [u32; M] = [0; M];
    return registers;
}

/*
    Aggregation Phase
*/
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
fn aggregation(fname: &str, registers: &mut [u32; M]) -> io::Result<()> {
    // Data Stream
    let file = File::open(fname)?;
    let lines = BufReader::new(file).lines();

    // Read lines into memory one by one
    for line in lines {

        let line_str = line?;
        let words = line_str.split(" ");

        for word in words {
            let hash: u32 = h32(word);
            
            // Get first P bits of the hash
            let idx = hash >> (32 - P);

            // Get rest of bits
            let w = hash & (u32::pow(2, 32 - P) - 1);

            // This is currently preventing us from using u64 hash
            registers[idx as usize] = std::cmp::max(registers[idx as usize], leading_zeros(w));
        }
    }

    Ok(())
}

/*
    Compute Result
*/
fn result_computation(registers: &mut [u32; M]) -> f64 {
    let mut e = estimate(registers);
    if e < 0.4 * (M as f64) {
        // Number of zero registers
        let v = num_zero_registers(registers);
        if v != 0 {
            e = linear_counting(v);
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

/*
    Main
    TODO: delete main function and keep file as library
          put tests in separate dir
*/
fn main() -> io::Result<()> {

    // Initialization Phase
    let mut registers: [u32; M] = init_registers();

    // Aggregation Phase
    aggregation("war_and_peace.txt", &mut registers)?;

    // Result Computation Phase
    let e = result_computation(&mut registers);

    println!("Estimated number of unique words: {}", e);

    Ok(())
}

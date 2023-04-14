/*
    From HLL++ paper:
    4 <= p <= 16 (u8)
    p1 <= 64 (u8)
    m = 2^p (u16)
    v = number of registers (m) set to 0 (u16)
    e = estimated bias (float64?)
    x = 64 bit hash (u64)
    k = ? gonna give u16 for now
    sparse_list = list of k values, so u16 arr?
*/

fn h() {

}

fn linear_counting(m: u16, v: u16) {
    // pass
}

fn threshold(p: u8) {
    // pass
}

fn estimate_base(e: f64, p: u8) {
    // pass
}

fn encode_hash(x: u64, p: u8, p1: u8) {
    // pass
}

fn get_index(k:u16, p:u8) {
    // pass
}

fn decode_hash(k: u16, p: u8, p1: u8) {
    // pass
}

fn to_normal(sparse_list: &[u16], p: u8, p1: u8) {
    // pass
}

fn merge(a: &[u16], b: &[u16]) {
    // pass
}

fn decode_sparse(a: &[u16]) {
    // pass
}

fn initialization() {
    // pass
}

fn aggregation() {
    // pass
}

fn result_computation() {
    // pass
}


// eventually move all main functions to tests so 
// we can use this file as a library source
fn main() {
    println!("Hello World!");
}

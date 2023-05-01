use rusthll::hll;

#[test]
fn war_and_peace() {
        let mut hll_struct = hll::HyperLogLog::new(6);
        // Aggregation Phase
        hll_struct.aggregation("war_and_peace.txt");
        println!("Registers: {:?}", hll_struct.registers);

        // Result Computation Phase
        let e = hll_struct.result_computation();
    
        assert_eq!(6563.224275190175, e);
}

#[test]
fn fingerprints() {
    let mut hll_struct = hll::HyperLogLog::new(6);
    // hll_struct.registers = [19, 22, 18, 20, 19, 18, 18, 19, 21, 21, 20, 18, 20, 18, 22, 19, 21, 19, 18, 20, 20, 17, 18, 19, 23, 19, 17, 22, 19, 19, 18, 18, 19, 20, 19, 19, 20, 22, 21, 18, 19, 20, 18, 19, 17, 19, 19, 22, 18, 20, 21, 17, 22, 20, 18, 21, 20, 20, 23, 19, 20, 18, 19, 22].to_vec();
    hll_struct.aggregation("fingerprints.txt");
    println!("Registers: {:?}", hll_struct.registers);

    let e = hll_struct.result_computation();

    assert_eq!(21444590.231436618, e);
}

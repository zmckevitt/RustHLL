use rusthll::hll;

#[test]
fn war_and_peace() {
        let mut hll_struct = hll::HyperLogLog::new(6);
        // Aggregation Phase
        hll_struct.aggregation("war_and_peace.txt");

        // Result Computation Phase
        let e = hll_struct.result_computation();
    
        assert_eq!(8000.0, e);
}

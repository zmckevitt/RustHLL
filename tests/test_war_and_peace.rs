use rusthll::hll;

#[test]
fn test() {
        let mut hll_struct = hll::HyperLogLog::new(6);
        // Aggregation Phase
        hll_struct.aggregation("war_and_peace.txt");

        // Result Computation Phase
        let e = hll_struct.result_computation();
    
        assert_eq!(7113.163086847374, e);
}
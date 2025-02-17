pub fn create_seed(x: u64) -> String {
    let max = u64::MAX;
    (max - (max % x)).to_string()
}
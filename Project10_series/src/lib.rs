pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut digits_series: Vec<String> = Vec::new();
    for i in 0..digits.len() {
        if i + len > digits.len() {
            break;
        }
        digits_series.push(digits.chars().skip(i).take(len).collect());
    }
    if len == 0 {
        digits_series.push("".to_string()); //Added for weird test case
    }
    digits_series
}

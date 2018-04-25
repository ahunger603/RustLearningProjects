pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut digits_series: Vec<String> = Vec::new();
    while digits.len() >= len {
        digits_series.push(digits.chars().take(len).collect());
    }
    digits_series
}

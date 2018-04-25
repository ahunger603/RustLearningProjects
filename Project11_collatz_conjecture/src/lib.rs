pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut steps = 0;
    let mut x = n;
    while x != 1 {
        if x % 2 == 0 {
            x /= 2;
        } else {
            x = 3*x + 1;
        }
        steps += 1;
    }
    Some(steps)
}

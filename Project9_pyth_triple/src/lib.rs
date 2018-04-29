const TARGET: u32 = 1000;
pub fn find() -> Option<u32> {
    for a in 1..TARGET-1 {
        for b in a..TARGET-1 {
            if a + b < TARGET {
                let c = ((a*a + b*b) as f64).sqrt();
                if c.floor() == c {
                    if a + b + c as u32 == TARGET {
                        return Some(a * b * c as u32);
                    }
                }
            }
        }
    }
    None
}


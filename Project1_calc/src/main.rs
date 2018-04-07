use std::io::Write;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn lcm(n: u64, m: u64) -> u64 { 
    (n * m) / gcd(n, m) 
}

fn calc(op_key: String, numbers: &mut Vec<u64>) -> Result<u64, &str> {
    if numbers.len() == 0 {
        return Err("No numbers");
    }

    let op: Box<fn(u64, &u64) -> u64> = match &op_key as &str {
        "sum" => Box::new(|mut sum: u64, &x: &u64| { sum += x; sum }),

        "product" => Box::new(|mut product: u64, &x: &u64| { product *= x; product }),

        "gcd" => Box::new(|mut gcd_acc: u64, &x: &u64| { gcd_acc = gcd(gcd_acc, x); gcd_acc }),

        "lcm" => Box::new(|mut lcm_acc: u64, &x: &u64| { lcm_acc = lcm(lcm_acc, x); lcm_acc }),

        _ => {
            writeln!(std::io::stderr(), "Invalid operation! Must be: sum, product, gcd or lcm").unwrap();
            std::process::exit(1);
        }
    };
    Ok(numbers.iter().skip(1).fold(numbers[0], *op))
}

fn main() {
    let mut op_key = String::from("");
    let mut numbers: Vec<u64> = Vec::new();
    for arg in std::env::args().skip(1) {
        if op_key == "" {
            op_key = String::from(arg);
        } else {
            numbers.push(u64::from_str(&arg)
                        .expect("error parsing argument"));
        }
    }

    match calc(op_key, &mut numbers) {
        Err(_) => std::process::exit(0),
        Ok(v) => println!("{}", v),
    }
}

#[test]
fn test_calc_invalid_inputs() {
    
}
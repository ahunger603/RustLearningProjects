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

fn vec_gcd(numbers: &mut Vec<u64>) {
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn calc(op: String, numbers: &mut Vec<u64>) {
    match &op as &str {
        "sum" => println!("not implemented"),
        "product" => println!("not implemented"),
        "gcd" => vec_gcd(numbers),
        "lcm" => println!("not implemented"),
        _ => {
            writeln!(std::io::stderr(), "Invalid operation! Must be: sum, product, gcd or lcm").unwrap();
            std::process::exit(1);
        }
    }
}

fn main() {
    let mut operation = String::from("");
    let mut numbers: Vec<u64> = Vec::new();

    for arg in std::env::args().skip(1) {
        if operation == "" {
            operation = String::from(arg);
        } else {
            numbers.push(u64::from_str(&arg)
                        .expect("error parsing argument"));
        }
    }

    if operation == "" {
        writeln!(std::io::stderr(), "Usage: gcd OPERATION_NAME NUMBERS ...").unwrap();
        std::process::exit(1);
    }

    if numbers.len() == 0 {
        std::process::exit(0);
    }

    calc(operation, &mut numbers);
}
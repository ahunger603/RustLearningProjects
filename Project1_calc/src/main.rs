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
    let op: Box<fn(u64, &u64) -> u64> = match &op_key as &str {
        "sum" => Box::new(|mut sum: u64, &x: &u64| {
            sum += x;
            sum
        }),

        "product" => Box::new(|mut product: u64, &x: &u64| {
            product *= x;
            product
        }),

        "gcd" => Box::new(|mut gcd_acc: u64, &x: &u64| {
            gcd_acc = gcd(gcd_acc, x);
            gcd_acc
        }),

        "lcm" => Box::new(|mut lcm_acc: u64, &x: &u64| {
            lcm_acc = lcm(lcm_acc, x);
            lcm_acc
        }),

        _ => return Err("Invalid Op"),
    };

    if numbers.len() == 0 {
        return Err("No numbers");
    }
    Ok(numbers.iter().skip(1).fold(numbers[0], *op))
}

fn main() {
    let mut op_key = String::from("");
    let mut numbers: Vec<u64> = Vec::new();
    for arg in std::env::args().skip(1) {
        if op_key == "" {
            op_key = String::from(arg);
        } else {
            numbers.push(u64::from_str(&arg).expect("error parsing argument"));
        }
    }

    match calc(op_key, &mut numbers) {
        Err("Invalid Op") => writeln!(std::io::stderr(),"Invalid operation! Must be: sum, product, gcd or lcm").unwrap(),
        Err(_) => std::process::exit(0),
        Ok(v) => println!("{}", v),
    }
}

#[test]
fn test_calc_invalid_inputs() {
    assert!(calc("sum".to_string(), &mut Vec::new()).is_err()); //No numbers
    assert!(calc("xyz".to_string(), &mut vec![1, 2, 3, 4]).is_err()); //Invalid operation
}

#[test]
fn test_calc_sum() {
    assert!(calc("sum".to_string(), &mut vec![10, 10, 10]).unwrap() == 30);
    assert!(calc("sum".to_string(), &mut vec![1, 2, 3]).unwrap() == 6);
    assert!(calc("sum".to_string(), &mut vec![5]).unwrap() == 5);
}

#[test]
fn test_calc_product() {
    assert!(calc("product".to_string(), &mut vec![1, 1, 1, 1, 1, 1, 1]).unwrap() == 1);
    assert!(calc("product".to_string(), &mut vec![0, 9999]).unwrap() == 0);
    assert!(calc("product".to_string(), &mut vec![5, 5, 2]).unwrap() == 50);
    assert!(calc("product".to_string(), &mut vec![2]).unwrap() == 2);
}

#[test]
fn test_calc_gcd() {
    assert!(calc("gcd".to_string(), &mut vec![125, 45, 30]).unwrap() == 5);
    assert!(calc("gcd".to_string(), &mut vec![3, 9, 12]).unwrap() == 3);
    assert!(calc("gcd".to_string(), &mut vec![8, 16, 32]).unwrap() == 8);
    assert!(calc("gcd".to_string(), &mut vec![64]).unwrap() == 64);
}

#[test]
fn test_calc_lcm() {
    assert!(calc("lcm".to_string(), &mut vec![10, 100, 1000]).unwrap() == 1000);
    assert!(calc("lcm".to_string(), &mut vec![31, 10]).unwrap() == 310);
    assert!(calc("lcm".to_string(), &mut vec![6]).unwrap() == 6);
}

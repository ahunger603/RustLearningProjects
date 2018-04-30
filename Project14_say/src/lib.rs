fn get_num_chunks(n: u64) -> Vec<u64> {
    let mut chunks: Vec<u64> = Vec::new();
    let n_str = n.to_string();
    let mut remaining = n_str.len();
    while remaining > 0 {
        let chunk_range = if remaining >= 3 {3} else {remaining};
        match n_str.get(remaining-chunk_range..remaining).unwrap().parse::<u64>() {
            Ok(chunk_val) => { chunks.push(chunk_val); },
            Err(_) => (),
        }
        remaining -= chunk_range;
    }
    chunks
}

fn get_chunk_order_string(order: u64) -> Option<&'static str> {
    match order {
        0 => None,
        1 => Some("thousand"),
        2 => Some("million"),
        3 => Some("billion"),
        4 => Some("trillion"),
        5 => Some("quadrillion"),
        6 => Some("quintillion"),
        _ => panic!("Order out of range!")
    }
}

fn get_decimal_string(dec: u64) -> &'static str {
    match dec {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => panic!("Decimal out of range!")
    }
}

fn get_tens_value_string(n: u64) -> String {
    if n >= 100 {
        panic!("Out of tens value range!")
    }
    let mut tens_string = String::new();
    if n > 10 && n < 20 {
        tens_string.push_str(match n {
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "ninteen",
            _ => panic!()
        });
    }
    else {
        if n >= 10 {
            let tens = n / 10;
            let remainder = n % 10;
            tens_string.push_str(match tens {
                1 => "ten",
                2 => "twenty",
                3 => "thirty",
                4 => "forty",
                5 => "fifty",
                6 => "sixty",
                7 => "seventy",
                8 => "eighty", 
                9 => "ninty",
                _ => panic!()
            });
            if remainder > 0 {
                tens_string.push('-');
                tens_string.push_str(get_decimal_string(remainder));
            }
        }
        else {
            if n > 0 {
                tens_string.push_str(get_decimal_string(n));
            }
        }
    }
    tens_string
}

fn get_chunk_value_string(n: u64) -> String {
    if n >= 1000 {
        panic!("Out of chunk value range!")
    }
    let mut val = n;
    let mut value_string = String::new();

    let hundreds = val / 100;
    if hundreds > 0 {
        value_string.push_str(get_decimal_string(hundreds));
        value_string.push_str(" hundred");
        val -= hundreds * 100;
        if val > 0 {
            value_string.push(' ');
        }
    }
    value_string.extend(get_tens_value_string(val).chars());

    value_string
}

pub fn encode(n: u64) -> String {
    let mut encoded = String::new();
    let chunks = get_num_chunks(n);
    let chunks_len = chunks.len();
    if chunks_len == 1 && chunks[0] == 0 {
        encoded.push_str(get_decimal_string(0));
    }
    else {
        for (i, value) in chunks.into_iter().rev().enumerate() {
            let order = chunks_len - (i + 1);
            if value > 0 {
                if i > 0 {
                    encoded.push(' ');
                }
                encoded.extend(get_chunk_value_string(value).chars());
                match get_chunk_order_string(order as u64) {
                    None => (),
                    Some(order_string) => {
                        encoded.push(' ');
                        encoded.push_str(order_string);
                    }
                }
            }
        }
    }
    encoded
}
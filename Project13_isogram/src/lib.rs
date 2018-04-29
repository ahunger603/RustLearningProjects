use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut contained_letters: HashMap<char, bool> = HashMap::new();
    for c in candidate.to_lowercase().chars() {
        match c {
            '-' | ' ' => continue,
            _ => {
                if contained_letters.contains_key(&c) {
                    return false;
                } else {
                    contained_letters.insert(c, true);
                }
            }
        }
    }
    true
}
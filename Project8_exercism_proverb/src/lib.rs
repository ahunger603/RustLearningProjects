pub fn build_proverb(list: Vec<&str>) -> String {
    let mut proverb: Vec<String> = Vec::new();
    if list.len() > 0 {
        let start = list[0];
        let mut last = start;
        for word in list.iter().skip(1) {
            proverb.push(format!("For want of a {} the {} was lost.", last, word));
            last = word;
        }
        proverb.push(format!("And all for the want of a {}.", start));
    }
    proverb.join("\n")
}


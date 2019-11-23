use std::collections::HashMap;

fn main() {
    let mut chars: Vec<HashMap<char, u16>> = vec![HashMap::new(); 8];
    for l in include_str!("input6").trim().lines() {
        for c in l.char_indices() {
            *chars[c.0].entry(c.1).or_insert(0) += 1;
        }
    }
    let (most, least): (String, String) = chars.iter()
        .map(|map| {
            let mut v: Vec<(&char, &u16)> = map.iter().collect();
            v.sort_by_key(|x| x.1);
            (*v.last().unwrap().0, *v.first().unwrap().0)
        }).unzip();
    println!("Most common: {}\nLeast common: {}", most, least);
}

use std::collections::BTreeMap;

fn main() {
    let mut sum: u32 = 0;
    for l in include_str!("input4").trim().replace("-","").lines() {
        let len = l.len();
        let (name, hash) = (&l[0..len-10], &l[len-6..len-1]);
        let mut m: BTreeMap<char, i8> = BTreeMap::new();
        for c in name.chars() {
            *m.entry(c).or_insert(0) -= 1;
        }
        let mut v: Vec<(&char, &i8)> = m.iter().collect();
        v.sort_by_key(|x| x.1);
        let key: String = v[..5].iter().map(|x| *x.0).collect();
        if key.as_str() == hash {
            let sid = l[len-10..len-7].parse().unwrap();
            if name.bytes()
                .map(|c| ((((c - 97) as u32 + sid) % 26) as u8 + 97) as char)
                .collect::<String>().contains("north") {
                    println!("North Pole objects store room: {}", sid);
                }
            sum += sid;
        }
    }
    println!("Sum of the sector IDs of the real rooms: {}", sum);
}

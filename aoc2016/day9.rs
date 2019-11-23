fn decompress(string: &str, version: usize) -> usize {
    let mut chars = string.chars();
    let mut size = 0;
    loop {
        let n = chars.by_ref().take_while(|&c| c != '(').count();
        size += n;
        let mark = chars.by_ref().take_while(|&c| c != ')').collect::<String>();
        if mark.len() == 0 { break }
        let m: Vec<usize> = mark.split('x').map(|z| z.parse().unwrap()).collect();
        let chunk = chars.by_ref().take(m[0]).collect::<String>();
        if version == 2 {
            size += m[1] * decompress(&chunk, 2);
        }
        else {
            let n: usize = m[0] * m[1];
            size += n;
        }
    }
    size
}

fn main() {
    let string = include_str!("input9").trim();
    println!("Version 1: {}", decompress(&string, 1));
    println!("version 2: {}", decompress(&string, 2));
}

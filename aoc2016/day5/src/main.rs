extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let key = "cxdnnyjw".as_bytes();
    let mut pw = String::new();
    let mut pw2 = vec!['\0'; 8];
    let mut h = Md5::new();
    (0..).skip_while(|x| {
        h.input(key);
        h.input(x.to_string().as_bytes());
        let hex = h.result_str();
        let c: Vec<char> = hex.chars().collect();
        if &hex[0..5] == "00000" {
            pw.push(c[5]);
            let z = c[5].to_digit(16).unwrap() as usize;
            if z < 8 && pw2[z] == '\0' {
                pw2[z] = c[6];
            }
        }
        h.reset();
        pw2.iter().any(|&x| x == '\0')
    }).next();
    println!("Password 1: {}", &pw[0..8]);
    println!("Password 2: {}", pw2.into_iter().collect::<String>());
}

#![feature(str_split_once)]
extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

extern crate parse_display;
use parse_display::FromStr;

#[derive(FromStr,PartialEq)]
enum Height {
    #[display("{0}in")]
    Inch(usize),
    #[display("{0}cm")]
    Cm(usize),
    #[display("{0}")]
    Undefied(usize),
}

#[derive(FromStr,PartialEq)]
enum Color {
    #[display("amb")]
    Amber,
    #[display("blu")]
    Blue,
    #[display("brn")]
    Brown,
    #[display("gry")]
    Gray,
    #[display("grn")]
    Green,
    #[display("hzl")]
    Hazel,
    #[from_str(regex="#(?P<0>[a-f0-9]+)")]
    Hex(String),
    #[display("oth")]
    Other,
    #[from_str(regex="(?P<0>[a-z0-9]+)")]
    Undefined(String)
}

#[derive(Default)]
struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: Option<Height>,
    hcl: Option<Color>,
    ecl: Option<Color>,
    pid: String,
    cid: Option<usize>
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr != 0
            && self.iyr != 0
            && self.eyr != 0
            && self.hgt != None
            && self.hcl != None
            && self.ecl != None
            && !self.pid.is_empty()
    }

    fn automatic_valid(&self) -> bool {
        self.byr >= 1920 && self.byr <= 2002
            && self.iyr >= 2010 && self.iyr <= 2020
            && self.eyr >= 2020 && self.eyr <= 2030
            && (match self.hgt {
                Some(Height::Inch(v)) => { v >= 59 && v <= 76 },
                Some(Height::Cm(v)) => { v >= 150 && v <= 193 },
                _ => false
            })
            && (match &self.hcl {
                Some(Color::Hex(v)) => { v.len() == 6 },
                _ => false
            })
            && (match self.ecl {
                None|Some(Color::Hex(_))|Some(Color::Undefined(_)) => false,
                _ => true
            })
            && self.pid.len() == 9 && (&self.pid).chars().all(|c| c.is_ascii_digit())
    }
}

fn main() {
    let mut passports = vec![];
    for batch in read_from_stdin().split("\n\n") {
        let mut pp = Passport::default();
        for entry in batch.split_whitespace() {
            if let Some((key,val)) = entry.split_once(':') {
                match key {
                    "byr" => { pp.byr = val.parse().unwrap(); }
                    "iyr" => { pp.iyr = val.parse().unwrap(); }
                    "eyr" => { pp.eyr = val.parse().unwrap(); },
                    "hgt" => { pp.hgt = val.parse().ok(); },
                    "hcl" => { pp.hcl = val.parse().ok(); },
                    "ecl" => { pp.ecl = val.parse().ok(); },
                    "pid" => { pp.pid = val.parse().unwrap(); },
                    "cid" => { pp.cid = val.parse().ok(); },
                    _ => {}
                };
            }
        }
        passports.push(pp);
    }
    println!("Part 1: {}", passports.iter().filter(|p| p.is_valid()).count());
    println!("Part 2: {}", passports.iter().filter(|p| p.automatic_valid()).count());
}

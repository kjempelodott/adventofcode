#[macro_use]
extern crate adventofcode2020;
use adventofcode2020::{into_lines,read_from_stdin};

extern crate parse_display;
use parse_display::FromStr;

#[derive(FromStr)]
#[display("{0}-{1}")]
struct Range(usize,usize);

#[derive(FromStr)]
#[display("{name}: {range1} or {range2}")] 
struct Field {
    pub name: String,
    range1: Range,
    range2: Range,
    #[from_str(default)]
    field: usize,
}

impl Field {
    fn in_range(&self, val: usize) -> bool {
        val >= self.range1.0 && val <= self.range1.1 ||
            val >= self.range2.0 && val <= self.range2.1
    }
}

fn main() {
    let input = into_lines(read_from_stdin());
    let mut fields: Vec<Field> = input[..20].iter()
        .map(|l| l.parse::<Field>().unwrap())
        .collect();
    let mut tickets = vec![];
    let error_rate = input[23..].iter()
        .fold(0, |sum,l| {
            let t: Vec<usize> = numbers!(l => usize);
            let s: usize = t.iter()
                .filter(|&v| fields.iter().find(|&f| f.in_range(*v)).is_none())
                .sum();
            if s == 0 { tickets.push(t) }
            sum + s
        });
    println!("Part 1: {}", error_rate);

    for c in 0..fields.len() {
        for p in 0..fields.len() {
            if tickets.iter().all(|t| fields[p].in_range(t[c])) {
                fields[p].field |= 1 << c;
            }
        }
    }    
    fields.sort_by_key(|f| f.field.count_ones());
    let mut found = 0;
    for f in fields.iter_mut() {
        let i = &mut f.field;
        let r = *i & found;
        found |= *i;
        *i = (*i^r).trailing_zeros() as usize;
    }

    let myticket: Vec<usize> = numbers!(input[21] => usize);
    println!("Part 2: {}", fields.iter()
             .enumerate()
             .filter(|(_,f)| f.name.starts_with("departure"))
             .map(|(i,_)| myticket[fields[i].field])
             .product::<usize>());
}

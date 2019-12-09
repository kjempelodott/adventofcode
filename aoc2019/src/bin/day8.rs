extern crate adventofcode2019;
use adventofcode2019::read_from_stdin;

fn main() {
    let input = read_from_stdin().into_bytes();
    let layers: Vec<_> = input.chunks(25*6).collect();
    let min0 = layers.iter().min_by_key(|layer| {
        layer.iter().filter(|&&c| c == 48).count()
    }).unwrap();
    println!("Part 1: {}", min0.iter().filter(|&&c| c == 49).count() * min0.iter().filter(|&&c| c == 50).count());
    
    println!("Part 2:");
    for i in 0..6 {
        let line = (0..25)
            .map(|j| {
                let l = layers.iter().skip_while(|l| l[i*25+j] == 50).next().unwrap();
                if l[i*25+j] == 48 { ' ' } else { '#' }
            })
            .collect::<String>();
        println!("{}", line);
    }
}

fn solve_part1(n: usize) -> String {
    let (mut elf1, mut elf2) = (0, 1);
    let mut sb = vec![3,7];
    while sb.len() < n+10 {
        let (s1, s2) = (sb[elf1], sb[elf2]);
        let sum = s1 + s2;
        if sum >= 10 {
            sb.push(1)
        }
        sb.push(sum % 10);
        elf1 = (elf1 + 1 + s1) % sb.len();
        elf2 = (elf2 + 1 + s2) % sb.len();
    }
    sb[n..n+10].iter().map(|&i| (i + 48) as u8 as char).collect::<String>()
}

fn solve_part2(m: &[usize]) -> usize {
    let k = m.len();
    let (mut elf1, mut elf2) = (3, 4);
    let mut sb = vec![3,7,1,0,1,0];
    loop {
        let (s1, s2) = (sb[elf1], sb[elf2]);
        let sum = s1 + s2;
        let mut l = sb.len();
        if sum >= 10 {
            sb.push(1);
            l += 1;
            if sb[l-k..l].iter().zip(m.iter()).all(|x| x.0 == x.1) {
                return l-k;
            }
        }
        sb.push(sum % 10);
        l += 1;
        if sb[l-k..l].iter().zip(m.iter()).all(|x| x.0 == x.1) {
            return l-k;
        }
        elf1 = (elf1 + 1 + s1) % l;
        elf2 = (elf2 + 1 + s2) % l;
    }
}

fn main() {
    println!("Part 1: {}", solve_part1(440231));
    println!("Part 2: {}", solve_part2(&[4,4,0,2,3,1]));
}

use std::collections::{BTreeMap,VecDeque};
use aoc2024::read_from_stdin;

#[derive(Clone,Debug)]
struct File(usize, usize);

fn main() {
    let diskmap: Vec<usize> = read_from_stdin().trim()
        .bytes().map(|b| (b - 48) as usize)
        .collect();

    let mut fsize = diskmap.iter().cloned().step_by(2).collect::<VecDeque<_>>();
    let mut slack = diskmap.iter().cloned().skip(1).step_by(2).collect::<VecDeque<_>>();

    let mut fragmented = vec![];
    let mut max_id = fsize.len() as isize - 1;
    let mut file = vec![];

    for id in 0..fsize.len() {
        if let Some(s) = fsize.pop_front() {
            fragmented.append(&mut vec![id; s]);
        }
        if let Some(mut s) = slack.pop_front() {
            while s > 0 {
                if let Some(part) = file.pop() {
                    fragmented.push(part);
                    s -= 1;
                    continue
                }
                if let Some(s) = fsize.pop_back() {
                    file = vec![max_id as usize; s];
                    max_id -= 1;
                    continue
                }
                break
            }
        }
    }
    println!("Part 1: {}", fragmented.iter().enumerate().map(|(i,&n)| i* n).sum::<usize>());

    let mut free_space: BTreeMap<usize, usize> = BTreeMap::new();
    let mut files: BTreeMap<usize, File> = BTreeMap::new();

    let (mut id, mut pos) = (0, 0);
    let mut it = diskmap.iter();
    while let Some(fsize) = it.next() {
        let slack = it.next().unwrap_or(&0);
        files.insert(pos, File(id, *fsize));
        pos += fsize;
        if *slack != 0 {
            free_space.insert(pos, *slack);
            pos += slack;
        }            
        id += 1;
    }

    let file_pos = files.keys().cloned().collect::<Vec<usize>>();
    for src in file_pos.iter().rev() {
        let file = files.remove(&src).unwrap();
        let mut dest = 0;
        if let Some(entry) = free_space.iter().take_while(|(p,_)| *p < src).find(|(_,s)| **s >= file.1) {
            dest = *entry.0;
        }
        if dest > 0 {
            let space = free_space.remove(&dest).unwrap();
            if space > file.1 {
                free_space.insert(dest + file.1, space - file.1);
            }
            files.insert(dest, file);
        }
        else {
            files.insert(*src, file);
        }
    }

    println!("Part 2: {}", files.iter()
             .map(|(p, file)| (*p..*p+file.1).map(|i| i * file.0).sum::<usize>())
             .sum::<usize>());
}

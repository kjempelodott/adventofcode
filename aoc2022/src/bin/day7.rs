extern crate aoc2022;
use aoc2022::read_from_stdin;

type Index = usize;
type File = (u64, String);

struct Dir {
    parent: Option<Index>,
    name: String,
    dirs: Vec<Index>,
    files: Vec<File>
}

impl Dir {
    fn new(parent: Option<Index>, name: &str) -> Self {
        Dir {
            parent: parent,
            name: name.into(),
            dirs: vec![],
            files: vec![]
        }
    }
    fn size(&self, index: &Vec<Dir>) -> u64 {
        self.files.iter().map(|t| t.0).sum::<u64>() +
            self.dirs.iter().map(|i| index[*i].size(&index)).sum::<u64>()
    }
}

fn main() {
    let root = Dir::new(None, "");
    let mut index = vec![root];
    let mut cid: Index = 0;

    for line in read_from_stdin().lines() {
        if line.starts_with('$') {
            let mut parts = line.split_ascii_whitespace().skip(1);
            if let Some("cd") = parts.next() {
                cid = match parts.next() {
                    Some("/") => 0,
                    Some("..") => index[cid].parent.unwrap(),
                    Some(dir) => *index[cid].dirs.iter().find(|&&i| index[i].name == dir).unwrap(),
                    None => unreachable!()
                };
            }
        }
        else {
            match line.split_once(" ") {
                Some(("dir", name)) => {
                    index.push(Dir::new(Some(cid), name));
                    let n = index.len() - 1;
                    index[cid].dirs.push(n);
                },
                Some((size, name)) => {
                    index[cid].files.push(
                        (size.parse::<u64>().unwrap(), name.into())
                    );
                }
                _ => {}
            };
        }
    }
    let mut sizes: Vec<(u64, Index)> = (0..index.len()).map(|i| (index[i].size(&index), i)).collect();
    let used_space: u64 = sizes[0].0;
    let to_free = 30000000 - (70000000 - used_space);
    sizes.sort();
    println!("Part 1: {}", sizes.iter().map(|t| t.0).take_while(|s| *s <= 100000).sum::<u64>());
    println!("Part 2: {}", sizes.iter().find(|t| t.0 >= to_free).unwrap().0);
}


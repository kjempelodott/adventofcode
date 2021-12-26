#[macro_use]
extern crate aoc2021;
use aoc2021::read_from_stdin;

#[derive(Debug,Hash,PartialEq,PartialOrd,Clone,Copy)]
struct Span(isize,isize);
impl std::ops::BitAnd for Span {
    type Output = Option<Self>;
    fn bitand(self, rhs: Self) -> Self::Output {
        let (a, b) = if rhs < self { (rhs, self) } else { (self, rhs) };
        if a.1 > b.0 {
            return Some(Span(b.0, a.1.min(b.1)))
        }
        None
    }
}

#[derive(Debug,Hash,PartialEq,PartialOrd,Clone,Copy)]
struct Cuboid(Span,Span,Span);
impl std::ops::BitAnd for Cuboid {
    type Output = Option<Self>;
    fn bitand(self, rhs: Self) -> Self::Output {
        let ix = (self.0 & rhs.0)?;
        let iy = (self.1 & rhs.1)?;
        let iz = (self.2 & rhs.2)?;
        Some(Cuboid(ix,iy,iz))
    }
}
impl std::ops::BitXor for Cuboid {
    type Output = Option<Vec<Self>>;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let iq = (self & rhs)?;
        let mut cuboids = vec![];
        if iq.0.0 > self.0.0 {
            cuboids.push(Cuboid(
                Span(self.0.0, iq.0.0),
                self.1,
                self.2
            ));
        }
        if iq.0.1 < self.0.1 {
            cuboids.push(Cuboid(
                Span(iq.0.1, self.0.1),
                self.1,
                self.2
            ));
        }   
        if iq.1.0 > self.1.0 {
            cuboids.push(Cuboid(
                Span(self.0.0.max(iq.0.0), self.0.1.min(iq.0.1)),
                Span(self.1.0, iq.1.0),
                self.2
            ));
        }
        if iq.1.1 < self.1.1 {
            cuboids.push(Cuboid(
                Span(self.0.0.max(iq.0.0), self.0.1.min(iq.0.1)),
                Span(iq.1.1, self.1.1),
                self.2
            ));
        }   
        if iq.2.0 > self.2.0 {
            cuboids.push(Cuboid(
                Span(self.0.0.max(iq.0.0), self.0.1.min(iq.0.1)),
                Span(self.1.0.max(iq.1.0), self.1.1.min(iq.1.1)),
                Span(self.2.0, iq.2.0)
            ));
        }
        if iq.2.1 < self.2.1 {
            cuboids.push(Cuboid(
                Span(self.0.0.max(iq.0.0), self.0.1.min(iq.0.1)),
                Span(self.1.0.max(iq.1.0), self.1.1.min(iq.1.1)),
                Span(iq.2.1, self.2.1)
            ));
        }
        Some(cuboids)        
    }
}
impl Cuboid {
    fn points(&self) -> isize {
        (self.0.1 - self.0.0)*(self.1.1 - self.1.0)*(self.2.1 - self.2.0)
    }
    fn inner_points(&self) -> isize {
        (self.0.1.max(-50).min(50) - self.0.0.min(50).max(-50))*
            (self.1.1.max(-50).min(50) - self.1.0.min(50).max(-50))*
            (self.2.1.max(-50).min(50) - self.2.0.min(50).max(-50))
    }
}

fn main() {
    let mut cuboids: Vec<Cuboid> = vec![];
    for line in read_from_stdin().lines() {
        let r: Vec<_> = numbers!(line => isize);
        let x = Span(r[0], r[1]+1);
        let y = Span(r[2], r[3]+1);
        let z = Span(r[4], r[5]+1);
        let q = Cuboid(x,y,z);
        if line.starts_with("on") && cuboids.is_empty() {
            cuboids.push(q);
        }
        else {
            cuboids = cuboids.iter()
                .fold(vec![], |mut acc, &c| {
                    if let Some(z) = c ^ q {
                        acc.extend(z);
                    }
                    else {
                        acc.push(c);
                    }
                    acc
                });      
            if line.starts_with("on") {
                cuboids.push(q);
            }
        }
    }
    println!("Part 1: {}", cuboids.iter()
             .map(|c| c.inner_points())
             .sum::<isize>());
    println!("Part 2: {}", cuboids.iter()
             .map(|c| c.points())
             .sum::<isize>());
}

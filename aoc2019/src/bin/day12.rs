#[macro_use]
extern crate adventofcode2019;
use adventofcode2019::read_from_stdin;
use adventofcode2019::metric::Point3D;

#[derive(Clone,Debug)]
struct Moon {
    pos: Point3D,
    velo: Point3D
}

fn simulate(moons: &mut Vec<Moon>) {
    for i in 0..4 {
        let mut moon0 = moons[i].clone();
        for j in (i+1)..4 {
            let ref mut moon1 = moons[j];
            if moon0.pos.x > moon1.pos.x {
                moon0.velo.x -= 1;
                moon1.velo.x += 1;
            }
            else if moon0.pos.x < moon1.pos.x {
                moon0.velo.x += 1;
                moon1.velo.x -= 1;
            }
            // else moon0.pos.x == moon1.pos.x
            if moon0.pos.y > moon1.pos.y {
                moon0.velo.y -= 1;
                moon1.velo.y += 1;
            }
            else if moon0.pos.y < moon1.pos.y {
                moon0.velo.y += 1;
                moon1.velo.y -= 1;
            }
            // else moon0.pos.y == moon1.pos.y
            if moon0.pos.z > moon1.pos.z {
                moon0.velo.z -= 1;
                moon1.velo.z += 1;
            }
            else if moon0.pos.z < moon1.pos.z {
                moon0.velo.z += 1;
                moon1.velo.z -= 1;
            }
            // else moon0.pos.z == moon1.pos.z
        }
        moon0.pos.mov(&moon0.velo);
        moons[i] = moon0;
    }
}

fn main() {
    let mut moons = numbers!(read_from_stdin() => isize)
        .chunks(3)
        .map(|p| Moon { pos: Point3D::new(p[2],p[1],p[0]), velo: Point3D::default() })
        .collect::<Vec<Moon>>();

    for _t in 0..1000 {
        simulate(&mut moons);
    }
    println!("Part 1: {}", moons.iter()
             .map(|m|
                  (m.pos.x.abs() + m.pos.y.abs() + m.pos.z.abs()) *
                  (m.velo.x.abs() + m.velo.y.abs() + m.velo.z.abs()))
             .sum::<isize>());
}

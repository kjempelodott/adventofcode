#[macro_use]
extern crate adventofcode2018;
extern crate regex;
use adventofcode2018::read_from_stdin;
use regex::Regex;

#[derive(Debug,PartialEq,Clone)]
enum DamageType {
    Radiation,
    Fire,
    Cold,
    Bludgeoning,
    Slashing
}       
use DamageType::*;

#[derive(Debug,PartialEq,Clone)]
enum Army {
    ImmuneSystem,
    Infection,
    Unset
}       

impl From<&str> for DamageType {
    fn from(string: &str) -> Self {
        match string.trim() {
            "radiation"   => Radiation,
            "fire"        => Fire,
            "cold"        => Cold,
            "bludgeoning" => Bludgeoning,
            "slashing"    => Slashing,
            _ => unreachable!()
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
struct Group {
    army: Army,
    units: usize,
    hp: usize,
    dmg: usize,
    initiative: usize,
    dmg_type: DamageType,
    weaknesses: Vec<DamageType>,
    immunities: Vec<DamageType>,
}

impl Group {
    fn power(&self) -> usize {
        self.units * self.dmg
    }

    fn effective_power(&self, target: &Group) -> usize {
        if target.immunities.contains(&self.dmg_type) {
            return 0
        }
        if target.weaknesses.contains(&self.dmg_type) {
            2 * self.power()
        } else {
            self.power()
        }
    }

    fn attack(&self, target: &Group) -> usize {
        let dmg_dealt = self.effective_power(&target);
        target.units.saturating_sub(dmg_dealt / target.hp)
    }
}

fn battle(mut groups: Vec<Group>) -> (usize, Army) {
    let mut total_units = groups.iter().map(|g| g.units).sum::<usize>();
    loop {
        groups = groups.into_iter().filter(|g| g.units > 0).collect();
        if !groups.iter().any(|g| g.army == Army::ImmuneSystem) {
            let army_size = groups.iter()
                .filter(|g| g.army == Army::Infection)
                .map(|g| g.units)
                .sum::<usize>();
            return (army_size, Army::Infection);
        }
        else if !groups.iter().any(|g| g.army == Army::Infection) {
            let army_size = groups.iter()
                .filter(|g| g.army == Army::ImmuneSystem)
                .map(|g| g.units)
                .sum::<usize>();
            return (army_size, Army::ImmuneSystem);
        }
        
        // Target selection phase
        // ----------------------
        // Groups with higher power choose first
        groups.sort_by_key(|g| g.power());
        let mut selected = vec![];
        let mut target_map = vec![];
        for idx in (0..groups.len()).rev() {
            let mut attacker = &groups[idx];
            // Filter out friendly groups and targets already selected
            let mut targets: Vec<usize> = (0..groups.len())
                .filter(|&i| groups[i].army != attacker.army)
                .filter(|&i| attacker.effective_power(&groups[i]) != 0)
                .filter(|i| !selected.contains(i))
                .collect();
            targets.sort_by_key(|&i| {
                let target = &groups[i];
                (attacker.effective_power(&target),
                 target.power())
            });
            if let Some(&j) = targets.last() {
                selected.push(j);
                target_map.push((idx, j));
            }
        }

        // Attack phase
        // -------------
        // Groups with higher intitiative attack first
        target_map.sort_by_key(|&(a,_)| groups[a].initiative);
        for (a,t) in target_map.into_iter().rev() {
            let units_left = groups[a].attack(&groups[t]);
            groups[t].units = units_left;
        }

        let units = groups.iter().map(|g| g.units).sum::<usize>();
        if total_units == units {
            return (0, Army::Unset);
        }
        total_units = units;
    }
}

fn main() {
    let re_dmg = Regex::new(r"(\w+) damage").unwrap();
    let re_weak = Regex::new(r"weak to (\w+(,\s\w+)*)").unwrap();
    let re_imm = Regex::new(r"immune to (\w+(,\s\w+)*)").unwrap();

    let mut groups: Vec<Group> = vec![];
    let mut army = Army::ImmuneSystem;
    for line in read_from_stdin().lines().skip(1) {
        let numbers = numbers!(line => usize);
        if numbers.len() != 4 {
            army = Army::Infection;
            continue;
        }
        let dmg = DamageType::from(re_dmg.captures(&line)
                                   .unwrap().get(1).unwrap().as_str());

        let weak = re_weak.captures(&line).map_or(vec![], |m| {
            m.get(1).unwrap().as_str()
                .split(", ")
                .map(|t| DamageType::from(t))
                .collect::<Vec<DamageType>>()
        });
        let imm = re_imm.captures(&line).map_or(vec![], |m| {
            m.get(1).unwrap().as_str()
                .split(", ")
                .map(|t| DamageType::from(t))
                .collect::<Vec<DamageType>>()
        });
        let group = Group {
            army       : army.clone(),
            units      : numbers[0],
            hp         : numbers[1],
            dmg        : numbers[2],
            initiative : numbers[3],
            dmg_type   : dmg,
            weaknesses : weak,
            immunities : imm,
        };
        groups.push(group);
    }

    // Groups with higher intitiative choose first
    groups.sort_by_key(|g| g.initiative);
    let mut groups_clone = groups.clone();
    let (size, _) = battle(groups_clone);
    println!("Part 1: {}", size);

    for boost in 1.. {
        groups_clone = groups.clone();
        for group in groups_clone.iter_mut() {
            if group.army == Army::ImmuneSystem {
                group.dmg += boost;
            }
        }
        let (size, winner) = battle(groups_clone);
        if winner == Army::ImmuneSystem {
            println!("Part 2: {}", size);
            break
        }
    }
}

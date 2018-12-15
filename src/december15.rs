#[macro_use]
extern crate maplit;

use std::collections::{BTreeMap, HashMap, HashSet};
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Unit {
    Elf,
    Goblin,
}

#[derive(Clone, Eq, PartialEq)]
enum Field {
    Wall,
    Cavern,
}

type Map = Vec<Vec<Field>>;
type Position = (i16, i16);
type Units = BTreeMap<Position, (Unit, i32)>;

static HP: i32 = 200;

fn parse(lines: &[String]) -> Result<(Map, Units)> {
    let mut units = BTreeMap::new();
    let mut map = vec![vec![Field::Cavern; lines[0].len()]; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if j > map[i].len() {
                Err(format!("Line {} too wide!", i + 1))?
            } else if c == '#' {
                map[i][j] = Field::Wall;
            } else if c == 'E' {
                units.insert((i as i16, j as i16), (Unit::Elf, HP));
            } else if c == 'G' {
                units.insert((i as i16, j as i16), (Unit::Goblin, HP));
            } else if c != '.' {
                Err(format!("Could not parse character: `{}`", c))?;
            }
        }
    }

    Ok((map, units))
}

fn ascii_art(map: &Map, units: &Units) -> String {
    let mut s = "".to_owned();
    for (i, row) in map.iter().enumerate() {
        for (j, field) in row.iter().enumerate() {
            match units.get(&(i as i16, j as i16)) {
                Some((Unit::Elf, _)) => s.push('E'),
                Some((Unit::Goblin, _)) => s.push('G'),
                None => match field {
                    Field::Wall => s.push('#'),
                    Field::Cavern => s.push('.'),
                },
            }
        }
        s.push('\n');
    }
    s
}

/// Find an adjacent enemy unit to attack (if any).
fn in_range((i, j): Position, unit: Unit, units: &Units) -> Option<Position> {
    let mut pos = None;
    let mut hp = HP + 1;
    for &(ti, tj) in &[(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
        if let Some((tunit, thp)) = units.get(&(ti, tj)) {
            if *tunit != unit && *thp < hp {
                pos = Some((ti, tj));
                hp = *thp;
            }
        }
    }
    pos
}

/// Find the next position to walk to.
fn next_pos(
    (i, j): Position,
    unit: Unit,
    units: &Units,
    map: &Map,
) -> Option<Position> {
    // Find all goals.
    let mut goals = HashSet::new();
    for (&(ti, tj), &(tunit, _)) in units {
        if tunit != unit {
            for &(ati, atj) in
                &[(ti - 1, tj), (ti, tj - 1), (ti, tj + 1), (ti + 1, tj)]
            {
                if !units.contains_key(&(ati, atj))
                    && map[ati as usize][atj as usize] == Field::Cavern
                {
                    goals.insert((ati, atj));
                }
            }
        }
    }

    if goals.is_empty() {
        return None;
    }

    let start = (i, j);

    let mut closed_set = HashSet::new();
    let mut open_set = hashset! {(i, j)};
    let mut came_from = HashMap::new();
    let mut costs = hashmap! {(i, j) => 0};

    while !open_set.is_empty() {
        // Find position with least cost so far.
        let mut pos = None;
        let mut cost = std::i16::MAX;
        for p in open_set.iter() {
            let cost_ = costs[p];
            if cost_ < cost || cost_ == cost && *p < pos.unwrap() {
                cost = cost_;
                pos = Some(*p);
            }
        }

        let (i, j) = pos.unwrap();
        open_set.remove(&(i, j));
        closed_set.insert((i, j));

        if goals.contains(&(i, j)) {
            // Arrived at goal. Retrace steps to arrive at first step.
            let mut first = (i, j);
            while let Some(p) = came_from.get(&first) {
                if *p == start {
                    return Some(first);
                }
                first = *p;
            }

            return Some(first);
        }

        for &(ni, nj) in &[(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
            if closed_set.contains(&(ni, nj))
                || units.contains_key(&(ni, nj))
                || map[ni as usize][nj as usize] != Field::Cavern
            {
                continue;
            }

            let tentative_cost = cost + 1;

            if !open_set.contains(&(ni, nj)) {
                open_set.insert((ni, nj));
            } else if tentative_cost >= costs[&(ni, nj)] {
                continue;
            }

            came_from.insert((ni, nj), (i, j));
            costs.insert((ni, nj), tentative_cost);
        }
    }

    None
}

/// Perform one battle step.
fn step(map: &Map, units: Units, elf_ap: i32, goblin_ap: i32) -> (Units, bool) {
    let mut new_units = units.clone();
    let mut died = HashSet::new();

    for (mut position, (unit, _)) in units {
        // Check if the battle has ended.
        let num_elves = new_units
            .iter()
            .map(|(_, (u, _))| u)
            .filter(|u| **u == Unit::Elf)
            .count();
        if num_elves == new_units.len() || num_elves == 0 {
            return (new_units, false);
        }

        // Check if the unit has already died.
        if died.contains(&position) {
            continue;
        }

        // Get current hp of unit.
        let (_, hp) = new_units[&position];

        // In range of target? Don't move.
        if in_range(position, unit, &new_units) == None {
            if let Some(npos) = next_pos(position, unit, &new_units, &map) {
                new_units.remove(&position);
                position = npos;
                new_units.insert(position, (unit, hp));
            }
        }

        if let Some((ti, tj)) = in_range(position, unit, &new_units) {
            let mut remove_target = false;

            {
                let (_, thp) = new_units.get_mut(&(ti, tj)).unwrap();
                if unit == Unit::Elf {
                    *thp -= elf_ap;
                } else {
                    *thp -= goblin_ap;
                }
                if *thp <= 0 {
                    remove_target = true;
                }
            }

            if remove_target {
                new_units.remove(&(ti, tj));
                died.insert((ti, tj));
            }
        }
    }

    (new_units, true)
}

fn combat(map: &Map, units: &Units, elf_ap: i32) -> (i32, Units) {
    let mut units: Units = units.to_owned();
    let mut num_rounds = 0;

    loop {
        let (nunits, full_round) = step(&map, units, elf_ap, 3);
        units = nunits;

        if full_round {
            num_rounds += 1
        } else {
            break;
        }
    }

    (num_rounds, units)
}

fn part1(map: &Map, units: &Units) -> i32 {
    let (num_rounds, units) = combat(map, units, 3);
    num_rounds * units.iter().map(|(_, (_, hp))| hp).sum::<i32>()
}

fn cheat(map: &Map, units: &Units) -> i32 {
    let num_elves = units
        .iter()
        .map(|(_, (u, _))| u)
        .filter(|u| **u == Unit::Elf)
        .count();

    let mut elf_attack_power = 0;
    loop {
        elf_attack_power += 1;
        let (num_rounds, units) = combat(map, units, elf_attack_power);

        let num_elves_ = units
            .iter()
            .map(|(_, (u, _))| u)
            .filter(|u| **u == Unit::Elf)
            .count();

        println!(
            "Attack power: {},\tsurviving elves: {}/{}",
            elf_attack_power, num_elves_, num_elves
        );

        if num_elves == num_elves_ {
            break num_rounds * units.iter().map(|(_, (_, hp))| hp).sum::<i32>();
        }
    }
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december15.txt")?;
    let (map, units) = parse(&lines)?;
    println!("{}", ascii_art(&map, &units));
    println!("Part 1: {:?}", part1(&map, &units));
    println!("Part 2: {:?}", cheat(&map, &units));

    Ok(())
}

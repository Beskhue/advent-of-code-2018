extern crate regex;
use std::collections::{HashMap, HashSet};
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Group {
    id: usize,
    units: i64,
    hitpoints: i64,
    weaknesses: HashSet<String>,
    immunities: HashSet<String>,
    attack_power: i64,
    attack_type: String,
    initiative: i64
}

impl std::hash::Hash for Group {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

fn parse(lines: &[String]) -> Result<(HashMap<usize, Group>, HashMap<usize, Group>)> {
    let re = regex::Regex::new(
        r"^(?P<units>[0-9]+) units each with (?P<hp>[0-9]+) hit points (?:\((?P<defense>.+)\) )?with an attack that does (?P<ap>[0-9]+) (?P<atype>[a-z]+) damage at initiative (?P<initiative>[0-9]+)$",
    )?;

    let mut immune_army = HashMap::new();
    let mut infection_army = HashMap::new();

    let mut n = 0;
    let mut in_immune = true;
    for line in lines {
        if line == "Immune System:" || line == "" {
            continue;
        } else if line == "Infection:" {
            in_immune = false;
            continue;
        }

        let captures = re
            .captures(line)
            .ok_or_else(|| format!("String `{}` does not parse", line))?;

        let units = captures.name("units")
            .ok_or("Parse err")?
            .as_str()
            .parse::<i64>()?;
        let hp = captures.name("hp")
            .ok_or("Parse err")?
            .as_str()
            .parse::<i64>()?;
        let ap = captures.name("ap")
            .ok_or("Parse err")?
            .as_str()
            .parse::<i64>()?;
        let initiative = captures.name("initiative")
            .ok_or("Parse err")?
            .as_str()
            .parse::<i64>()?;

        let mut weaknesses = HashSet::new();
        let mut immunities = HashSet::new();

        if let Some(defense) = captures.name("defense") {
            for defense in defense.as_str().split("; ") {
                if &defense[..7] == "weak to" {
                    weaknesses = defense[8..].split(", ")
                        .map(|s| s.to_owned())
                        .collect()
                } else if &defense[..9] == "immune to" {
                    immunities = defense[10..].split(", ")
                        .map(|s| s.to_owned())
                        .collect()
                }
            }
        }

        let group = Group {
            id: n,
            units: units,
            hitpoints: hp,
            weaknesses: weaknesses,
            immunities: immunities,
            attack_power: ap,
            attack_type: captures.name("atype").ok_or("Parse err")?.as_str().to_string(),
            initiative: initiative
        };

        if in_immune {
            immune_army.insert(n, group);
        } else {
            infection_army.insert(n, group);
        }

        n += 1;
    }

    Ok((immune_army, infection_army))
}

fn effective_power(group: &Group) -> i64 {
    group.units * group.attack_power
}

fn damage(attacker: &Group, defender: &Group) -> i64 {
    effective_power(attacker) * 
    if defender.weaknesses.contains(&attacker.attack_type) {
        2
    } else if defender.immunities.contains(&attacker.attack_type) {
        0
    } else {
        1
    }
}

/// Returns a tuple of a boolean and number of units alive.
/// Boolean true indicates the immune army has won, false the infection.
fn battle(mut immune_army: HashMap<usize, Group>, mut infection_army: HashMap<usize, Group>) -> (bool, i64) {
    let mut prev_all_groups = Vec::new();

    loop {
        let mut immune_groups = immune_army.values().cloned().collect::<Vec<Group>>();
        let mut infection_groups = infection_army.values().cloned().collect::<Vec<Group>>();

        let mut immune_targets = HashMap::new();
        let mut infection_targets = HashMap::new();

        immune_groups.sort_unstable_by_key(|g| {
            (effective_power(g), g.initiative)
        });

        for attacking_group in immune_groups.clone().iter().rev() {
            infection_groups.sort_unstable_by_key(|g| {
                (damage(&attacking_group, g), effective_power(g), g.initiative)
            });

            for defending_group in infection_groups.clone().iter().rev() {
                if damage(&attacking_group, &defending_group) == 0 {
                    continue;
                }

                if !immune_targets.values().collect::<Vec<_>>().contains(&&defending_group.id) {
                    immune_targets.insert(attacking_group.id, defending_group.id);
                    break;
                }
            }
        }

        infection_groups.sort_unstable_by_key(|g| {
            (effective_power(g), g.initiative)
        });

        for attacking_group in infection_groups.clone().iter().rev() {
            immune_groups.sort_unstable_by_key(|g| {
                (damage(&attacking_group, g), effective_power(g), g.initiative)
            });

            for defending_group in immune_groups.clone().iter().rev() {
                if damage(&attacking_group, &defending_group) == 0 {
                    continue;
                }

                if !infection_targets.values().collect::<Vec<_>>().contains(&&defending_group.id) {
                    infection_targets.insert(attacking_group.id, defending_group.id);
                    break;
                }
            }
        }

        let mut all_groups = immune_groups.iter().map(|g| (g.initiative, true, g.id, g.units)).collect::<Vec<(i64, bool, usize, i64)>>();
        all_groups.append(&mut infection_groups.iter().map(|g| (g.initiative, false, g.id, g.units)).collect());
        all_groups.sort_unstable_by_key(|&(i, _, _, _)| i);

        if prev_all_groups == all_groups {
            // Stalemate
            break (false, -1)
        }

        prev_all_groups = all_groups.clone();

        for &(_, is_immune_army, id, _) in all_groups.iter().rev() {
            let (attacking_army, defending_army, targets) =
                if is_immune_army {
                    (&mut immune_army, &mut infection_army, &immune_targets)
                } else {
                    (&mut infection_army, &mut immune_army, &infection_targets)
                };

            let attacker = attacking_army.get(&id);
            if let Some(attacker) = attacker {
                if !targets.contains_key(&id) {
                    continue
                }

                let mut defender = defending_army.get(&targets[&id]).unwrap().clone();

                let dmg = damage(attacker, &defender);
                let num_kill = dmg / defender.hitpoints;

                defender.units -= num_kill;

                if defender.units <= 0 {
                    defending_army.remove(&defender.id);
                } else {
                    defending_army.insert(defender.id, defender);
                }
            }
        }

        if infection_army.len() == 0 {
            break (true, immune_army.values().map(|g| g.units).sum())
        } else if immune_army.len() == 0 {
            break (false, infection_army.values().map(|g| g.units).sum())
        }
    }
}

fn find_boost(immune_army: HashMap<usize, Group>, infection_army: HashMap<usize, Group>) -> i64 {   
    let mut boost = 0;

    loop {
        let ia = immune_army.iter().map(|(key, g)| {
            let mut g = g.clone();
            g.attack_power += boost;
            return (*key, g);
        }).collect::<HashMap<usize,Group>>();

        let (won, alive) = battle(ia, infection_army.clone());

        if won {
            break alive;
        }

        boost += 1;
    }
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december24.txt")?;
    let (immune_army, infection_army) = parse(&lines)?;

    println!("Part 1: {:?}", battle(immune_army.clone(), infection_army.clone()));
    println!("Part 2: {:?}", find_boost(immune_army, infection_army));

    Ok(())
}

use std::collections::BTreeSet;
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;
type Plant = bool;
type Rule = ([Plant; 5], Plant);

fn parse(lines: &[String]) -> Result<(Vec<Plant>, Vec<Rule>)> {
    let plants = lines[0][15..].chars().map(|c| c == '#').collect();
    let rules: Vec<Rule> = lines[2..]
        .iter()
        .map(|s| {
            let mut rule = [false; 5];

            for (idx, c) in s[0..5].chars().enumerate() {
                rule[idx] = c == '#'
            }

            let outcome = s.chars().nth(9).ok_or("Invalid rule")? == '#';
            Ok((rule, outcome))
        })
        .collect::<Result<Vec<Rule>>>()?;

    Ok((plants, rules))
}

fn game_of_plants(plants: &[Plant], rules: &[Rule], epochs: u64) -> i64 {
    let mut all_plants = BTreeSet::new();

    let (mut lbound, mut rbound) = (0i64, (plants.len() - 1) as i64);
    for (idx, _) in plants.iter().enumerate().filter(|(_, p)| **p) {
        all_plants.insert(idx as i64);
    }

    // Run all generations (so long and thanks for all the cycles) or until we
    // find a pattern fixpoint (two identical generations modulo pot numbers).
    for epoch in 1..=epochs {
        let mut all_plants_ = BTreeSet::new();
        let (mut nlbound, mut nrbound) = (std::i64::MAX, std::i64::MIN);
        for k in lbound - 2..=rbound + 2 {
            let slice = [
                all_plants.contains(&(k - 2)),
                all_plants.contains(&(k - 1)),
                all_plants.contains(&k),
                all_plants.contains(&(k + 1)),
                all_plants.contains(&(k + 2)),
            ];
            if *rules
                .iter()
                .filter(|(rule, _)| slice == *rule)
                .map(|(_, outcome)| outcome)
                .nth(0)
                .unwrap()
            {
                all_plants_.insert(k);
                if k < nlbound {
                    nlbound = k;
                }
                if k > nrbound {
                    nrbound = k;
                }
            }
        }

        if rbound - lbound == nrbound - nlbound {
            // Field sizes are identical, so the pattern might have shifted.
            if all_plants
                .iter()
                .map(|k| k - lbound)
                .collect::<BTreeSet<_>>()
                .symmetric_difference(
                    &all_plants_
                        .iter()
                        .map(|k| k - nlbound)
                        .collect::<BTreeSet<_>>(),
                )
                .count()
                == 0
            {
                // Pattern has shifted. Construct the final configuration.
                all_plants = all_plants
                    .iter()
                    .map(|k| {
                        k + (nlbound - lbound) * (epochs - epoch + 1) as i64
                    })
                    .collect();
                break;
            }
        }

        lbound = nlbound;
        rbound = nrbound;

        all_plants = all_plants_;
    }

    all_plants.iter().sum()
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december12.txt")?;
    let (plants, rules) = parse(&lines)?;

    println!("Part 1: {}", game_of_plants(&plants, &rules, 20));
    println!(
        "Part 2: {}",
        game_of_plants(&plants, &rules, 50_000_000_000u64)
    );

    Ok(())
}

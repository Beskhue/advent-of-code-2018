extern crate regex;
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Unit {
    Positive(char),
    Negative(char)
}

trait UnitType {
    fn get_unit_type(&self) -> char;
    fn is_opposite(&self, other: &Self) -> bool;
}

impl UnitType for Unit {
    fn get_unit_type(&self) -> char {
        match *self {
            Unit::Positive(c) => c,
            Unit::Negative(c) => c
        }
    }

    fn is_opposite(&self, other: &Unit) -> bool {
        self.get_unit_type() == other.get_unit_type() && *self != *other
    }
}

fn get_polymer(lines: &[String]) -> Result<Vec<Unit>> {
    if lines.len() != 1 {
        return Err("Expected only one line of input".into())
    }

    Ok(
        lines[0].chars()
            .map(|c| if c.is_ascii_uppercase() { Unit::Positive(c.to_ascii_lowercase()) } else { Unit::Negative(c) })
            .collect()
    )
}

fn react(polymer: &[Unit]) -> Vec<&Unit> {
    polymer.iter().fold(Vec::new(), |mut reacted, unit| {
        if !reacted.is_empty() && unit.is_opposite(reacted.last().unwrap()) {
            reacted.pop();
        } else {
            reacted.push(unit);
        }
        reacted
    })
}

fn remove_unit(polymer: &[Unit], unit_type: char) -> Vec<Unit> {
    polymer.iter().filter(|u| u.get_unit_type() != unit_type).cloned().collect()
}

fn part2(polymer: &[&Unit]) -> usize {
    let ascii_iter = (0..26).map(|x| (x + b'a') as char);
    ascii_iter.fold(polymer.len(), |shortest, char| {
        std::cmp::min(
            shortest, 
            react(
                &remove_unit(
                    &polymer.iter().cloned().cloned().collect::<Vec<_>>(),
                    char)
            ).len()
        )
    })
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december05.txt")?;
    let polymer = get_polymer(&lines)?;
    let reacted = react(&polymer);

    println!("Part 1: {:#?}", reacted.len());
    println!("Part 2: {:#?}", part2(&reacted));
    
    Ok(())
}

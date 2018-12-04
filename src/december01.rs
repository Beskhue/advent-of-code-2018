use std::collections::BTreeMap;
mod utils;

fn find_duplicate_frequency(numbers: &[i32]) -> i32 {
    let mut frequencies = BTreeMap::new();
    let mut found = false;
    let mut frequency = 0;

    while !found {
        for number in numbers {
            frequency += number;

            if frequencies.contains_key(&frequency) {
                found = true;
                break;
            }

            frequencies.insert(frequency, true);
        }
    }

    frequency
}

fn main() {
    let lines: Vec<String> = utils::lines_from_file("input/december01.txt").unwrap();
    let numbers: Vec<i32> = lines
        .into_iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let sum: i32 = (&numbers).iter().sum();
    let dup = find_duplicate_frequency(&numbers);

    println!("Part 1: {:#?}", sum);
    println!("Part 2: {:#?}", dup)
}

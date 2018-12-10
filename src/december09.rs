mod utils;
use std::collections::{VecDeque, HashMap};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn rotate(table: &mut VecDeque<u64>, rot: i32) {
    let mut rot = rot;
    if rot > 0 {
        while rot > 0 {
            let ele = table.pop_back().unwrap();
            table.push_front(ele);
            rot -= 1;
        }
    } else {
        let ele = table.pop_front().unwrap();
        table.push_back(ele);
    }
}

fn play(num_players: u64, last_value: u64) -> u64 {
    let mut scores = HashMap::new();

    // Use a deque for quick insertion and retrieval at the front *and* back.
    let mut table: VecDeque<u64> = vec![0].iter().cloned().collect();
    table.reserve_exact((last_value * 22 / 23) as usize);
    let mut n = 1;
    loop {
        if n % 23 == 0 { // Scoring move.
            rotate(&mut table, 7);
            let value = table.pop_back().unwrap();
            *scores.entry(n % num_players).or_insert(0) += value + n;
            rotate(&mut table, -1);
        } else { // Placing move
            rotate(&mut table, -1);
            table.push_back(n);
        }

        if n == last_value {
            break *scores.values().max().unwrap()
        }

        n += 1;
    }
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december09.txt")?;
    let config: Vec<&str> = lines[0].split(" ").collect();
    let num_players = config[0].parse::<u64>()?;
    let last_marble = config[6].parse::<u64>()?;

    println!("Part 1: {:?}", play(num_players, last_marble));
    println!("Part 1: {:?}", play(num_players, last_marble * 100));
    
    Ok(())
}

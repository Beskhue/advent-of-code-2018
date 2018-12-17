use std::collections::HashMap;
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;
type Position = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Clay,
    SettledWater,
    ActiveWater,
}

fn parse(lines: &[String]) -> Result<HashMap<Position, Tile>> {
    let mut map = HashMap::new();
    for line in lines {
        let parts = line.split(", ").collect::<Vec<_>>();
        let i = parts
            .get(0)
            .and_then(|p| p.get(2..))
            .ok_or("Malformed data")?
            .parse::<i32>()?;
        let js = parts
            .get(1)
            .and_then(|p| p.get(2..))
            .ok_or("Malformed data")?
            .split("..")
            .map(|x| Ok(x.parse::<i32>()?))
            .collect::<Result<Vec<_>>>()?;

        for j in js[0]..=js[1] {
            if line.chars().nth(0).unwrap() == 'y' {
                map.insert((j, i), Tile::Clay);
            } else {
                map.insert((i, j), Tile::Clay);
            }
        }
    }

    Ok(map)
}

fn simulate(
    map: &mut HashMap<Position, Tile>,
    spring_x: i32,
    spring_y: i32,
    max_y: i32,
) {
    if spring_y > max_y {
        return;
    }

    let mut tiles = Vec::new();
    let mut walled_in = true;
    static LEFT: i32 = 0;
    static RIGHT: i32 = 1;

    let (mut spread_left, mut spread_right) = (true, true);
    let mut dx = 0_i32;
    while spread_left || spread_right {
        for &dir in &[LEFT, RIGHT] {
            if dir == LEFT && !spread_left
                || dir == RIGHT && !spread_right
                || dir == RIGHT && dx == 0
            {
                continue;
            }

            let x = spring_x + if dir == LEFT { -dx } else { dx };
            let tile = map.get(&(x, spring_y)).cloned();
            if tile != None {
                assert!(dx != 0, "Spring is in clay!");
                if dir == LEFT {
                    spread_left = false;
                } else {
                    spread_right = false;
                }
                continue;
            }

            tiles.push((x, spring_y));
            let mut below = map.get(&(x, spring_y + 1)).cloned();
            if below == None {
                simulate(map, x, spring_y + 1, max_y);
                below = map.get(&(x, spring_y + 1)).cloned();
            }

            if below == Some(Tile::ActiveWater) || spring_y == max_y {
                if dx == 0 || dir == LEFT {
                    spread_left = false;
                }
                if dx == 0 || dir == RIGHT {
                    spread_right = false;
                }
                walled_in = false;
            }
        }

        dx += 1;
    }

    for tile in tiles {
        map.insert(
            tile,
            if walled_in {
                Tile::SettledWater
            } else {
                Tile::ActiveWater
            },
        );
    }
}

fn ascii_art(map: &HashMap<Position, Tile>) -> String {
    let min_x = map.keys().map(|&(x, _)| x).min().unwrap();
    let max_x = map.keys().map(|&(x, _)| x).max().unwrap();
    let min_y = map.keys().map(|&(_, y)| y).min().unwrap();
    let max_y = map.keys().map(|&(_, y)| y).max().unwrap();

    let mut s = "".to_string();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match map.get(&(x, y)) {
                Some(Tile::Clay) => s.push('#'),
                Some(Tile::SettledWater) => s.push('~'),
                Some(Tile::ActiveWater) => s.push('|'),
                None => s.push(' '),
            }
        }
        s.push('\n');
    }
    s
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december17.txt")?;
    let mut map = parse(&lines)?;
    let min_y = map.keys().map(|&(_, y)| y).min().unwrap();
    let max_y = map.keys().map(|&(_, y)| y).max().unwrap();
    simulate(&mut map, 500, min_y, max_y);
    println!("{}", ascii_art(&map));

    let count_all = map
        .values()
        .filter(|&&v| v == Tile::ActiveWater || v == Tile::SettledWater)
        .count();
    let count_settled =
        map.values().filter(|&&v| v == Tile::SettledWater).count();

    println!("Part 1: {}", count_all);
    println!("Part 2: {}", count_settled);

    Ok(())
}

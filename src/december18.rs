mod utils;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Open,
    Trees,
    Lumberyard,
}

type Map = Vec<Vec<Tile>>;

fn parse(lines: &[String]) -> Result<Map> {
    let mut map =
        vec![
            vec![Tile::Open; lines.get(0).ok_or("Malformed input")?.len()];
            lines.len()
        ];

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            map[i][j] = match c {
                '.' => Tile::Open,
                '|' => Tile::Trees,
                '#' => Tile::Lumberyard,
                _ => Err("Malformed input")?,
            };
        }
    }

    Ok(map)
}

fn count(map: &Map, (i, j): (usize, usize)) -> (u8, u8, u8) {
    let (i, j) = (i as i32, j as i32);
    let (mut open, mut trees, mut lumberyard) = (0, 0, 0);
    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0
                || i + di < 0
                || i + di >= map.len() as i32
                || j + dj < 0
                || j + dj >= map[0].len() as i32
            {
                continue;
            }
            match map[(i + di) as usize][(j + dj) as usize] {
                Tile::Open => open += 1,
                Tile::Trees => trees += 1,
                Tile::Lumberyard => lumberyard += 1,
            }
        }
    }
    (open, trees, lumberyard)
}

fn run(mut map: Map, iterations: u64) -> i32 {
    let mut visited = HashMap::new();

    for epoch in 0..iterations {
        if visited.contains_key(&map) {
            let first_epoch = visited[&map];
            let idx = (iterations - epoch) % (epoch - first_epoch);

            map = visited
                .iter()
                .filter(|&(&_, &v): &(&Map, &u64)| v == first_epoch + idx)
                .map(|(k, _)| k.clone())
                .nth(0)
                .unwrap();
            break;
        } else {
            visited.insert(map.clone(), epoch);
        }

        let mut map_ = map.clone();
        for (i, row) in map_.iter_mut().enumerate() {
            for (j, tile) in row.iter_mut().enumerate() {
                let (_open, trees, lumberyard) = count(&map, (i, j));
                if *tile == Tile::Open && trees >= 3 {
                    *tile = Tile::Trees;
                } else if *tile == Tile::Trees && lumberyard >= 3 {
                    *tile = Tile::Lumberyard;
                } else if *tile == Tile::Lumberyard
                    && (trees == 0 || lumberyard == 0)
                {
                    *tile = Tile::Open;
                }
            }
        }
        map = map_;
    }

    let trees: i32 = map
        .iter()
        .map(|row| row.iter().filter(|&&t| t == Tile::Trees).count() as i32)
        .sum();
    let lumberyards: i32 = map
        .iter()
        .map(|row| {
            row.iter().filter(|&&t| t == Tile::Lumberyard).count() as i32
        })
        .sum();

    trees * lumberyards
}

fn ascii_art(map: &Map) -> String {
    let mut s = "".to_owned();

    for row in map.iter() {
        for tile in row {
            match tile {
                Tile::Open => s.push(' '),
                Tile::Trees => s.push('|'),
                Tile::Lumberyard => s.push('#'),
            }
        }
        s.push('\n');
    }
    s
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december18.txt")?;
    let map = parse(&lines)?;
    println!("{}", ascii_art(&map));

    println!("Part 1: {:?}", run(map.clone(), 10));
    println!("Part 2: {:?}", run(map.clone(), 1_000_000_000));
    Ok(())
}

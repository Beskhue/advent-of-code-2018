mod utils;
use std::collections::BTreeMap;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;
type Map = Vec<Vec<char>>;
type Position = (usize, usize);
type Direction = i32;

fn parse(lines: &[String]) -> Result<(Map, BTreeMap<Position, Direction>)> {
    let mut map = Vec::new();
    let mut carts = BTreeMap::new();
    for (i, line) in lines.iter().enumerate() {
        let mut l = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let (m, cart) = match c {
                '^' => ('|', Some(0)),
                '>' => ('-', Some(90)),
                'v' => ('|', Some(180)),
                '<' => ('-', Some(270)),
                _ => (c, None),
            };

            l.push(m);
            if let Some(cart) = cart {
                carts.insert((i, j), cart);
            }
        }
        map.push(l);
    }

    Ok((map, carts))
}

fn run(
    map: Map,
    carts: BTreeMap<Position, Direction>,
) -> Result<(Option<Position>, Option<Position>)> {
    let mut carts = carts
        .iter()
        .map(|(&k, &v)| (k, (v, -90)))
        .collect::<BTreeMap<_, _>>();
    let mut first_crash = None;
    // Hide cursor for animation.
    print!("{}[?25l", 27 as char);
    loop {
        // Animate!
        println!("{}[2J{}", 27 as char, ascii_art(&map, &carts.iter().map(|(&k,&(d,_m))| (k,d)).collect())?);
        std::thread::sleep(std::time::Duration::from_millis(84));

        let mut carts_new = BTreeMap::new();
        for (&(i, j), &(dir, mem)) in carts.iter() {
            let (ni, nj) = if carts_new.contains_key(&(i, j)) {
                // Crashed.
                (i, j)
            } else {
                match dir {
                    0 => Ok((i - 1, j)),
                    90 => Ok((i, j + 1)),
                    180 => Ok((i + 1, j)),
                    270 => Ok((i, j - 1)),
                    _ => Err("Unknown direction"),
                }?
            };

            let t = map
                .get(ni)
                .and_then(|l| l.get(nj))
                .ok_or(format!("Cart derailed at {}, {}", nj, ni))?;

            let (dd, nmem) = match &t {
                '/' => (if dir == 0 || dir == 180 { 90 } else { -90 }, mem),
                '\\' => (if dir == 0 || dir == 180 { -90 } else { 90 }, mem),
                '+' => (
                    mem,
                    match mem {
                        -90 => 0,
                        0 => 90,
                        _ => -90,
                    },
                ),
                _ => (0, mem),
            };

            if carts_new.contains_key(&(ni, nj)) {
                // Crashed.
                carts_new.remove(&(ni, nj));
                if first_crash == None {
                    first_crash = Some((ni, nj));
                }
            } else {
                carts_new.insert((ni, nj), ((dir + dd + 360) % 360, nmem));
            }
        }

        carts = carts_new;

        if carts.len() <= 1 {
            break Ok((first_crash, carts.keys().nth(0).map(|&v| v)));
        }
    }
}

fn ascii_art(
    map: &Map,
    carts: &BTreeMap<Position, Direction>,
) -> Result<String> {
    let mut s = "".to_owned();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if let Some(d) = carts.get(&(i, j)) {
                s.push(match d {
                    0 => Ok('^'),
                    90 => Ok('>'),
                    180 => Ok('v'),
                    270 => Ok('<'),
                    _ => Err("Unknown direction"),
                }?);
            } else {
                s.push(map[i][j]);
            }
        }
        s.push('\n');
    }

    Ok(s)
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december13forever.txt")?;
    let (map, carts) = parse(&lines)?;
    let (p1, p2) = run(map, carts)?;

    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);

    Ok(())
}

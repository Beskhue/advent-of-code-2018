#[macro_use]
extern crate maplit;
use std::collections::{HashSet, VecDeque};
mod utils;

#[derive(Debug, PartialEq, Eq, Hash)]
enum RRegex {
    Concat(Vec<RRegex>),
    Options(Vec<RRegex>),
    Literal(char),
}

type Result<T> = std::result::Result<T, Box<std::error::Error>>;
type Position = (i32, i32);

fn parse(regex: &str) -> RRegex {
    fn parse_(mut regex: &str) -> (RRegex, &str) {
        let mut options: Vec<RRegex> = vec![];
        let mut concat: Vec<RRegex> = vec![];

        while !regex.is_empty() {
            match regex.chars().nth(0).unwrap() {
                '|' => {
                    options.push(RRegex::Concat(concat));
                    concat = vec![];
                    regex = &regex[1..];
                }
                '(' => {
                    let (r, remaining) = parse_(&regex[1..]);
                    concat.push(r);
                    regex = remaining;
                }
                ')' => {
                    regex = &regex[1..];
                    break;
                }
                c => {
                    concat.push(RRegex::Literal(c));
                    regex = &regex[1..];
                }
            }
        }

        options.push(RRegex::Concat(concat));
        (RRegex::Options(options), regex)
    }

    let (r, _) = parse_(regex);
    r
}

fn find_edges(regex: RRegex) -> HashSet<(Position, Position)> {
    let mut edges = HashSet::new();

    fn run_(
        regex: RRegex,
        mut positions: HashSet<Position>,
        edges: &mut HashSet<(Position, Position)>,
    ) -> HashSet<Position> {
        match regex {
            RRegex::Concat(rs) => {
                for r in rs {
                    positions = run_(r, positions, edges)
                }
            }
            RRegex::Options(rs) => {
                let mut positions_ = hashset! {};
                for r in rs {
                    positions_ = positions_
                        .union(&run_(r, positions.clone(), edges))
                        .cloned()
                        .collect();
                }
                positions = positions.union(&positions_).cloned().collect();
            }
            RRegex::Literal(c) => {
                let mut positions_ = hashset! {};
                for (x, y) in positions {
                    let (nx, ny) = match c {
                        'N' => (x, y - 1),
                        'E' => (x + 1, y),
                        'S' => (x, y + 1),
                        'W' => (x - 1, y),
                        _ => panic!(format!("Unexpected literal {}", c)),
                    };

                    positions_.insert((nx, ny));
                    edges.insert(((x, y), (nx, ny)));
                    edges.insert(((nx, ny), (x, y)));
                }
                positions = positions_;
            }
        }

        positions
    }

    run_(regex, hashset! {(0, 0)}, &mut edges);
    edges
}

fn longest_path(from: Position, edges: HashSet<(Position, Position)>) -> usize {
    let mut visited = hashmap! {from => 0};

    let mut queue = vec![(from, 0, hashset! {})];
    while !queue.is_empty() {
        let ((x, y), len, mut path) = queue.pop().unwrap();
        path.insert((x, y));

        for (dx, dy) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let new_p = (x + dx, y + dy);

            if edges.contains(&((x, y), new_p))
            // There is a door.
                && !path.contains(&new_p)
            // We haven't been here on this walk.
                && (!visited.contains_key(&new_p) || visited[&new_p] < len)
            // We have never been here after more doors on a different walk.
            {
                visited.insert(new_p, len + 1);
                queue.push((new_p, len + 1, path.clone()));
            }
        }
    }

    *visited.values().max().unwrap()
}

fn far_rooms(from: Position, edges: HashSet<(Position, Position)>) -> usize {
    let mut rooms = hashset! {};

    // Find all rooms.
    for (from, to) in edges.clone() {
        rooms.insert(from);
        rooms.insert(to);
    }

    let mut visited = hashset! {from};
    let mut deque = VecDeque::from(vec![(from, 0)]);

    // Find all rooms within 1000 doors.
    while !deque.is_empty() {
        let ((x, y), len) = deque.pop_front().unwrap();

        if len < 1000 {
            for (dx, dy) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
                let new_p = (x + dx, y + dy);

                if !visited.contains(&new_p) && edges.contains(&((x, y), new_p))
                {
                    visited.insert(new_p);
                    deque.push_back((new_p, len + 1));
                }
            }
        }
    }
    rooms.len() - (visited.len() - 1)
}

fn main() -> Result<()> {
    let re = utils::lines_from_file("input/december20.txt")?
        .get(0)
        .ok_or("Err")?
        .clone();
    let regex = parse(&re[1..re.len() - 1]);
    let edges = find_edges(regex);

    println!("Starting p1");
    println!("Part 1: {}", longest_path((0, 0), edges.clone()));
    println!("Part 2: {}", far_rooms((0, 0), edges));

    Ok(())
}

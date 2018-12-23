#[macro_use]
extern crate maplit;
use std::collections::{HashSet, HashMap};
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;
type Position = (u64, u64);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Tool {
    Neither,
    Torch,
    Climbing
}

fn erosion_level(geologic_index: u64, depth: u64) -> u64 {
    (geologic_index + depth) % 20183
}

fn geologic_index(
    (x, y): Position,
    target: Position,
    depth: u64,
    geo_idx: &mut HashMap<Position, u64>
) -> u64
{
    if geo_idx.contains_key(&(x, y)) {
        return geo_idx[&(x, y)];
    }

    let idx = 
        if (x, y) == (0, 0) {
            0
        } else if (x, y) == target {
            0
        } else if y == 0 {
            (x * 16807) % 20183
        } else if x == 0 {
            (y * 48271) % 20183
        } else {
            (
                erosion_level(geologic_index((x-1, y), target, depth, geo_idx), depth)
                * erosion_level(geologic_index((x, y-1), target, depth, geo_idx), depth)
            ) % 20183
        };

    geo_idx.insert((x, y), idx);
    idx
}

fn tile_type(
    position: Position,
    target: Position,
    depth: u64,
    geo_idx: &mut HashMap<Position, u64>
) -> u64
{
    erosion_level(geologic_index(position, target, depth, geo_idx), depth) % 3
}

fn total_risk_level(depth: u64, (tx, ty): Position) -> u64 {
    let mut geo_idx = hashmap!{};
    let mut risk = 0;

    for x in 0..=tx {
        for y in 0..=ty {
            risk += tile_type((x, y), (tx, ty), depth, &mut geo_idx);
        }
    }

    risk
}

fn shortest_path(depth: u64, target: Position) -> u64 {
    let (tx, ty) = target;
    let mut geo_idx = hashmap!{};

    let mut closed_set = HashSet::new();
    let mut open_set = hashset! {((0, 0), Tool::Torch)};
    let mut costs = hashmap! {((0, 0), Tool::Torch) => 0};

    while !open_set.is_empty() {
        // Find position with least cost so far.
        let mut pos = None;
        let mut heur_cost = std::u64::MAX;
        for &p in open_set.iter() {
            let ((nx, ny), t) = p;

            let mut f = if nx < tx { tx - nx } else { nx - tx }
                + if ny < ty { ty - ny } else { ny - ty };
            if t != Tool::Torch {
                f += 7;
            }

            let heur_cost_ = costs[&p] + f;
            if heur_cost_ < heur_cost {
                heur_cost = heur_cost_;
                pos = Some(p);
            }
        }

        let node = pos.unwrap();
        let ((x, y), tool) = node;
        open_set.remove(&node);
        closed_set.insert(node);

        let cost = costs[&node];

        if (x, y) == target && tool == Tool::Torch {
            return cost;
        }

        let mut neighbors = vec![
            ((x, y), Tool::Neither),
            ((x, y), Tool::Torch),
            ((x, y), Tool::Climbing),
            ((x+1, y), tool),
            ((x, y+1), tool)
        ];
        if x > 0 {
            neighbors.push(((x-1, y), tool));
        }
        if y > 0 {
            neighbors.push(((x, y-1), tool));
        }

        for &((nx, ny), ntool) in &neighbors {
            let next_node = ((nx, ny), ntool);
            if closed_set.contains(&next_node)
                || ((nx, ny), ntool) == node
            {
                continue;
            }

            let tile = tile_type((nx, ny), target, depth, &mut geo_idx);

            if tile == 0 && ntool == Tool::Neither {
                continue;
            } else if (nx, ny) != target && tile == 1 && ntool == Tool::Torch {
                continue;
            } else if tile == 2 && ntool == Tool::Climbing {
                continue;
            }

            let move_cost = if ntool != tool {
                7
            } else {
                1
            };

            let tentative_cost = cost + move_cost;

            if !open_set.contains(&next_node) {
                open_set.insert(next_node);
            } else if tentative_cost >= costs[&next_node] {
                continue;
            }

            costs.insert(next_node, tentative_cost);
        }
    }

    std::u64::MAX
}

fn main() -> Result<()> {
    let (depth, target): (u64, Position) = (11991, (6,797));
    //let (depth, target): (u64, Position) = (510, (10,10));

    println!("Part 1: {}", total_risk_level(depth, target));
    println!("Part 2: {}", shortest_path(depth, target));

    Ok(())
}

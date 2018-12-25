use std::collections::HashSet;
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Debug, Eq, PartialEq)]
struct Point4D<T> {
    x: T,
    y: T,
    z: T,
    w: T
}

impl <T> Point4D<T> {
    fn new(x: T, y: T, z: T, w: T) -> Point4D<T> {
        Point4D {
            x, y, z, w
        }
    }
}

impl Point4D<i64> {
    fn distance(&self, other: &Point4D<i64>) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
        + (self.z - other.z).abs()  + (self.w - other.w).abs()
    }
}

fn min_distance(ps1: &[&Point4D<i64>], ps2: &[&Point4D<i64>]) -> Option<i64> {
    if ps2.is_empty() {
        return None
    }
    ps1.iter()
        .map(|p1| {
            ps2.iter()
                .map(|p2| p1.distance(p2))
                .min()
                .unwrap()
        })
        .min()
}


fn parse_points(lines: &[String]) -> Result<Vec<Point4D<i64>>> {
    lines.iter()
        .map(|s| {
            let parts: Vec<&str> = s.split(",").collect();

            Ok(Point4D::new(
                parts[0].trim().parse()?,
                parts[1].trim().parse()?,
                parts[2].trim().parse()?,
                parts[3].trim().parse()?
            ))
        })
        .collect()
}

fn num_constellations(points: &[Point4D<i64>]) -> usize {
    let mut constellations = points.clone().iter().map(|p| vec![p]).collect::<Vec<_>>();

    let mut mutated = true;
    while mutated {
        let mut merged = HashSet::new();
        let mut new_constellations = Vec::new();

        for (idx, constellation) in constellations.iter().enumerate() {
            if !merged.contains(&idx) {
                let mut constellation = constellation.clone();

                for (idx_, constellation_) in constellations.iter().enumerate() {
                    if idx_ != idx && !merged.contains(&idx_) {
                        if min_distance(&constellation, constellation_) <= Some(3) {
                            constellation.append(&mut constellation_.clone());
                            merged.insert(idx_);
                        }
                    }
                }

                new_constellations.push(constellation);
            }
        }
        
        mutated = new_constellations != constellations;
        constellations = new_constellations;
    }

    constellations.len()
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december25.txt")?;
    let points = parse_points(&lines)?;

    println!("Part 1: {:#?}", num_constellations(&points));
    println!("Part 2: {:#?}", "Thanks Rudolph! ❤");
    
    Ok(())
}


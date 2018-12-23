extern crate euclid;
extern crate regex;
use euclid::Point3D;
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;
type Constellation = Vec<(Point3D<i64>, i64)>;

fn parse(lines: &[String]) -> Result<Constellation> {
    let re = regex::Regex::new(
        r"^pos=<\s*(-?[0-9]+),\s*(-?[0-9]+),\s*(-?[0-9]+)>, r=([0-9]+)$",
    )?;
    lines
        .iter()
        .map(|s| {
            let captures = re
                .captures(s)
                .ok_or_else(|| format!("String {} does not parse", s))?;

            let x1 = captures[1].parse::<i64>()?;
            let x2 = captures[2].parse::<i64>()?;
            let x3 = captures[3].parse::<i64>()?;
            let r = captures[4].parse::<i64>()?;

            Ok((Point3D::new(x1, x2, x3), r))
        })
        .collect()
}

fn in_range_largest(constellation: &Constellation) -> usize {
    let (pos, strength) = constellation.iter().max_by_key(|(_, s)| s).unwrap();

    constellation
        .iter()
        .filter(|(p, _)| {
            ((p.x - pos.x).abs() + (p.y - pos.y).abs() + (p.z - pos.z).abs())
                <= *strength
        })
        .count()
}

fn count_within_cube(
    constellation: &Constellation,
    origin: Point3D<i64>,
    size: i64,
) -> usize {
    let mut count = 0;

    for &(pos, strength) in constellation {
        let dist = (origin.x + size / 2 - pos.x).abs()
            + (origin.y + size / 2 - pos.y).abs()
            + (origin.z + size / 2 - pos.z).abs();

        if (dist - strength) / size <= 0 {
            count += 1
        }
    }

    count
}

fn closest_in_best_range(constellation: &Constellation) -> i64 {
    let mut min_x = constellation.iter().map(|(p, _)| p.x).min().unwrap();
    let mut max_x = constellation.iter().map(|(p, _)| p.x).max().unwrap();
    let mut min_y = constellation.iter().map(|(p, _)| p.y).min().unwrap();
    let mut max_y = constellation.iter().map(|(p, _)| p.y).max().unwrap();
    let mut min_z = constellation.iter().map(|(p, _)| p.z).min().unwrap();
    let mut max_z = constellation.iter().map(|(p, _)| p.z).max().unwrap();

    let sx = max_x - min_x;
    let sy = max_y - min_y;
    let sz = max_z - min_z;

    let size_ = std::cmp::max(std::cmp::max(sx, sy), sz);

    let mut size = 1i64;
    while size < size_ {
        size = size * 2;
    }

    loop {
        let mut best = 0;
        let mut candidate: Option<Point3D<i64>> = None;

        for x in (min_x..=max_x).step_by(size as usize) {
            for y in (min_y..=max_y).step_by(size as usize) {
                for z in (min_z..=max_z).step_by(size as usize) {
                    let origin = Point3D::new(x, y, z);
                    let count = count_within_cube(constellation, origin, size);
                    if count > best
                        || count == best
                            && (candidate == None
                                || origin.x.abs()
                                    + origin.y.abs()
                                    + origin.z.abs()
                                    < candidate.unwrap().x.abs()
                                        + candidate.unwrap().y.abs()
                                        + candidate.unwrap().z.abs())
                    {
                        best = count;
                        candidate = Some(origin);
                    }
                }
            }
        }

        let candidate = candidate.unwrap();

        if size == 1 {
            break candidate.x.abs() + candidate.y.abs() + candidate.z.abs();
        }

        min_x = candidate.x - size;
        max_x = candidate.x + size;
        min_y = candidate.y - size;
        max_y = candidate.y + size;
        min_z = candidate.z - size;
        max_z = candidate.z + size;

        size /= 2;
    }
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december23.txt")?;
    let constellation = parse(&lines)?;

    println!("Part 1: {}", in_range_largest(&constellation));
    println!("Part 2: {:?}", closest_in_best_range(&constellation));

    Ok(())
}

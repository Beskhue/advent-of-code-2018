extern crate regex;
extern crate euclid;
use euclid::{Rect, Point2D, Vector2D};
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;
type Sky = Vec<(Point2D<i64>, Vector2D<i64>)>;

fn parse(lines: &[String]) -> Result<Sky> {
    let re = regex::Regex::new(r"^position=<\s*(-?[0-9]+),\s*(-?[0-9]+)> velocity=<\s*(-?[0-9]+),\s*(-?[0-9]+)>$")?;
    lines.iter()
        .map(|s| {
            let captures = re.captures(s).ok_or_else(|| format!("String {} does not match", s))?;

            let x1 = captures[1].parse::<i64>()?;
            let x2 = captures[2].parse::<i64>()?;
            let v1 = captures[3].parse::<i64>()?;
            let v2 = captures[4].parse::<i64>()?;

            Ok((Point2D::new(x1, x2), Vector2D::new(v1, v2)))
        })
        .collect()
}

fn conserve_momentum(sky: &mut Sky) -> (Sky, i32) {
    let mut area = std::i64::MAX;
    let mut seconds = 0;

    loop {
        for (x, v) in sky.iter_mut() {
            *x += *v;
        }

        let bbox = Rect::from_points(sky.iter().map(|(x, _)| x));
        if bbox.size.area() > area {
            // If the bounding box increases in size; the previous
            // result was the goal. Backtrack and break.
            for (x, v) in sky.iter_mut() {
                *x -= *v;
            }

            break (sky.clone(), seconds)
        }

        seconds += 1;
        area = bbox.size.area();
    }
}

fn ascii_art(sky: Sky) -> String {
    let mut s = "".to_owned();
    let bbox = Rect::from_points(sky.iter().map(|(x, _)| x));
    for x2 in bbox.origin.y..=bbox.origin.y+bbox.size.height {
        for x1 in bbox.origin.x..=bbox.origin.x+bbox.size.   width {
            if sky.iter().any(|(x, _)| x.x == x1 && x.y == x2) {
                s.push_str("#");
            } else {
                s.push_str(" ");
            }
        }

        s.push_str("\n");
    }

    s
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december10.txt")?;
    let mut sky = parse(&lines)?;
    let (sky, seconds) = conserve_momentum(&mut sky);
    println!("Part 1:\n{}", ascii_art(sky));
    println!("Part 2: {}", seconds);
    
    Ok(())
}

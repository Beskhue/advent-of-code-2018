extern crate euclid;
use euclid::{Rect, Point2D, Size2D};
use std::collections::BTreeMap;
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn parse_coordinates(lines: &[String]) -> Result<Vec<Point2D<i32>>> {
	lines.iter()
		.map(|s| {
			let parts: Vec<&str> = s.split(", ").collect();
			Ok(Point2D::new(parts[0].parse()?, parts[1].parse()?))
		})
		.collect()
}

fn distance(coord: &Point2D<i32>, other_coord: &Point2D<i32>) -> i32 {
	(coord.x - other_coord.x).abs() + (coord.y - other_coord.y).abs()
}

fn part1(coords: &[Point2D<i32>]) -> i32 {
	let bbox_ = Rect::from_points(coords);
	let bbox = Rect::new(bbox_.origin, Size2D::new(bbox_.size.width+1, bbox_.size.height+1));
	let inside_box = bbox.inflate(-1, -1);

	let mut infinite = Vec::new();
	let mut counts = BTreeMap::new(); //vec![0; coords.len()];

	for x in bbox.origin.x..=bbox.origin.x + bbox.size.width {
		for y in bbox.origin.y..=bbox.origin.y + bbox.size.height {
			let point = Point2D::new(x, y);
			let mut closest = None;
			let mut min_distance = std::i32::MAX;
			for coord in coords.iter() {
				let dist = distance(coord, &point);
				if dist < min_distance {
					min_distance = dist;
					closest = Some(coord);
				} else if dist == min_distance {
					closest = None;
				}
			}

			if let Some(closest) = closest {
				let count = counts.entry((closest.x, closest.y)).or_insert(0);
				*count += 1;

				if !inside_box.contains(&point) {
					infinite.push(closest);
				}
			}
		}
	}

	coords.iter().fold(0, |max, coord| {
		if !infinite.contains(&coord) {
			std::cmp::max(max, counts[&(coord.x, coord.y)])
		} else {
			max
		}
	})
}

fn part2(coords: &[Point2D<i32>]) -> i32 {
	let bbox_ = Rect::from_points(coords);
	let bbox = Rect::new(bbox_.origin, Size2D::new(bbox_.size.width+1, bbox_.size.height+1));
	let mut size = 0;

	for x in bbox.origin.x..=bbox.origin.x + bbox.size.width {
		for y in bbox.origin.y..=bbox.origin.y + bbox.size.height {
			let point = Point2D::new(x, y);
			if coords.iter().fold(0, |acc, coord| acc + distance(coord, &point)) < 10_000 {
				size += 1;
			}
		}
	}
	size
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december06.txt")?;
    let coordinates = parse_coordinates(&lines)?;

    println!("Part 1: {:#?}", part1(&coordinates));
    println!("Part 2: {:#?}", part2(&coordinates));
    
    Ok(())
}


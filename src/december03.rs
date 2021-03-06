extern crate regex;
extern crate euclid;
use euclid::{Rect, Point2D, Size2D};
use std::collections::BTreeMap;
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

///Finds all overlaps between claims (including duplicate overlaps).
fn claim_overlaps(claims: &[Rect<u32>]) -> Vec<Rect<u32>> {
    let mut intersections = Vec::new();
    
    for (idx, rect1) in claims[1..].iter().enumerate() {
        for rect2 in claims[..=idx].iter() {
            if let Some(intersection) = rect1.intersection(rect2) {
                intersections.push(intersection)
            }
        }
    }
    
    intersections
}

/// Naively finds the overlap between claims by exhaustively tracking each
/// square inch in claim overlaps.
fn overlap_area_naive(claims: &[Rect<u32>]) -> u32 {
    let intersections = claim_overlaps(claims);
    
    // Naive solution:
    let mut overlaps = BTreeMap::new();
    for intersection in intersections {
        for i in intersection.min_y()..intersection.max_y() {
            for j in intersection.min_x()..intersection.max_x() {
                overlaps.insert((i, j), true);
            }
        }
    }

    overlaps.len() as u32
}

/// Finds the total overlap between claims by first finding all overlaps, then
/// calculating the area of the first overlap, and subsequently mapping all
/// remaining overlaps into four pieces: i.e., for each remaining overlap r,
/// we create:
/// - the part of r that is to the top of the current overlap;
/// - the part of r that is to the right of the current overlap;
/// - the part of r that is to the bottom of the current overlap; and
/// - the part of r that is to the left of the current overlap.
///
/// Assumes there is at least one claim.
fn overlap_area(claims: &[Rect<u32>]) -> u32 {
    let overlaps = claim_overlaps(claims);

    let bound = overlaps
        .iter()
        .fold(None, |bound, &claim| Some(bound.unwrap_or(claim).union(&claim)))
        .unwrap();
    
    fn area(bound: &Rect<u32>, overlaps: &[Rect<u32>]) -> u32 {
        if overlaps.is_empty() {
            0
        } else {
            let overlap = overlaps[0];
            let tail = &overlaps[1..];
            
            // Finds all intersections in the tail given a bound.
            let intersecting = |&bound: &Rect<_>| -> Vec<Rect<_>> {
                tail
                    .iter()
                    .filter(|o| bound.intersects(o))
                    .map(|o| bound.intersection(o).unwrap())
                    .collect()
            };
               
            /*
                If the current bounding box is of size 4x7, and we have an
                overlap of size 2x4 at (0,1), we have:
                ........
                .1111...
                .1111...
                ........

                and the bounding boxes we recurse with are:
                TTTTTTTT
                L1111RRR
                L1111RRR
                BBBBBBBB
            */
            
            let bound_top = Rect::from_points(&[
                Point2D::new(bound.min_x(), bound.min_y()),
                Point2D::new(bound.max_x(), overlap.min_y())
            ]);
            let bound_right = Rect::from_points(&[
                Point2D::new(overlap.max_x(), overlap.min_y()),
                Point2D::new(bound.max_x(), overlap.max_y())
            ]);
            let bound_bottom = Rect::from_points(&[
                Point2D::new(bound.min_x(), overlap.max_y()),
                Point2D::new(bound.max_x(), bound.max_y())
            ]);
            let bound_left = Rect::from_points(&[
                Point2D::new(bound.min_x(), overlap.min_y()),
                Point2D::new(overlap.min_x(), overlap.max_y())
            ]);
            

            // Sanity check.
            assert!(
                bound.intersection(&bound_top).unwrap_or(bound_top).size.area() +
                bound.intersection(&bound_right).unwrap_or(bound_right).size.area() +
                bound.intersection(&bound_bottom).unwrap_or(bound_bottom).size.area() +
                bound.intersection(&bound_left).unwrap_or(bound_left).size.area()
                == bound.size.area() - overlap.size.area()
            );
            
            overlap.size.area()
                + area(&bound_top, &intersecting(&bound_top))
                + area(&bound_right, &intersecting(&bound_right))
                + area(&bound_bottom, &intersecting(&bound_bottom))
                + area(&bound_left, &intersecting(&bound_left))
        }
    }
    
    area(&bound, &overlaps)
}

/// Finds the id of the (first) claim that overlaps no other claim.
/// Note that claim ids in the input monotonically increase by 1, starting at 1.
fn no_overlap(claims: &[Rect<u32>]) -> Option<usize> {
    for (idx, claim) in claims.iter().enumerate() {
        if !claims
            .iter()
            .enumerate()
            .filter(|(idx2, _)| idx != *idx2)
            .any(|(_, claim2)| claim.intersects(claim2))
        {
            return Some(idx+1);
        }
    }
    
    None
}

/// Parses a list of claim Strings into claim Rects.
fn parse_claims(lines: &[String]) -> Result<Vec<Rect<u32>>> {
    let re = regex::Regex::new(r"\D")?;  // Matches all non-digits.
    
    lines
        .iter()
        .map(|s| {
            // Replace all non-digits by spaces.
            let nstr = re.replace_all(s, " ");
            
            // Parse all numbers.
            let numbers = nstr
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().map_err(|e| e.into()))
                .collect::<Result<Vec<_>>>()?;
            
            Ok(Rect::new(
                Point2D::new(numbers[1], numbers[2]),
                Size2D::new(numbers[3], numbers[4])
            ))
        })
        .collect::<Result<Vec<_>>>()
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december03.txt")?;
    let claims = parse_claims(&lines)?;
    
    println!("Part 1 (naive):            {:#?}", overlap_area_naive(&claims));
    println!("Part 1 (divide & conquer): {:#?}", overlap_area(&claims));
    println!("Part 2: {:#?}", no_overlap(&claims).unwrap_or(0));
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let claims_str = vec![
            "#1 @ 1,3: 4x4".to_owned(),
            "#2 @ 3,1: 4x4".to_owned(),
            "#3 @ 5,5: 2x2".to_owned(),
            "#4 @ 6,6: 1x1".to_owned()
        ];
        let sizes = vec![
            (1,3,4,4),
            (3,1,4,4),
            (5,5,2,2),
            (6,6,1,1)
        ];
        
        let claims = parse_claims(&claims_str).unwrap();
        
        assert!(claims.len() == 4);
        for (idx, (x, y, width, height)) in sizes.iter().enumerate() {
            assert!(
                claims[idx].origin.x == *x
                && claims[idx].origin.y == *y
                && claims[idx].size.width == *width
                && claims[idx].size.height == *height
            );
        }
    }
    
    #[test]
    fn overlap() {
        let claims_str = vec![
            "#1 @ 1,3: 4x4".to_owned(),
            "#2 @ 3,1: 4x4".to_owned()
        ];
        
        let claims = parse_claims(&claims_str).unwrap();
        let overlap = claim_overlaps(&claims);
        
        assert!(overlap.len() == 1);
        assert!(
            overlap[0].origin.x == 3
            && overlap[0].origin.y == 3
            && overlap[0].size.width == 2
            && overlap[0].size.height == 2
        );
    }
}

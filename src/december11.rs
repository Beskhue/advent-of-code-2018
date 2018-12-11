#![feature(test)]
extern crate test;
extern crate euclid;
use euclid::{Rect, Point2D, Size2D};

static SERIAL_INPUT: i32 = 5153;

fn power(x: usize, y: usize) -> i32 {
    let x = x as i32;
    let y = y as i32;
    let rack_id = x + 10;
    (((rack_id * y + SERIAL_INPUT) * rack_id) % 1000) / 100 - 5
}

/// Uses a summed-area table.
fn max_power_summed_area_table(bbox: Rect<usize>, sizes: &[usize]) -> (Point2D<usize>, usize) {
    let mut max_area = (Point2D::new(0,0), 0);
    let mut max_power = std::i32::MIN;

    let mut grid = vec![vec![0; bbox.size.width+1]; bbox.size.height+1];

    for x in 1..=bbox.size.width {
        for y in 1..=bbox.size.height {
            grid[x][y] = power(x,y) + grid[x-1][y] + grid[x][y-1] - grid[x-1][y-1]
        }
    }

    for size in sizes {
        for x in bbox.origin.x..bbox.origin.x + bbox.size.width - (size - 1) {
            for y in bbox.origin.y..bbox.origin.y + bbox.size.height - (size - 1) {
                let power = grid[x-1][y-1] + grid[x+size-1][y+size-1] - grid[x+size-1][y-1] - grid[x-1][y+size-1];
                if power > max_power {
                    max_area = (Point2D::new(x, y), *size);
                    max_power = power;
                }
            }
        }
    }
    
    max_area
}

fn main() {
    println!("Part 1: {:?}", max_power_summed_area_table(Rect::new(Point2D::new(1,1),Size2D::new(300,300)), &[3]));
    println!("Part 2: {:?}", max_power_summed_area_table(Rect::new(Point2D::new(1,1),Size2D::new(300,300)), &(1..=300).collect::<Vec<_>>()));
}


#[cfg(test)]
mod tests {
    use euclid::{Rect, Point2D, Size2D};
    use test::Bencher;

    #[bench]
    fn max_summed_area_table(b: &mut Bencher) {
        b.iter(|| super::max_power_summed_area_table(Rect::new(Point2D::new(1,1),Size2D::new(300,300)), &(1..=300).collect::<Vec<_>>()));
    }
}

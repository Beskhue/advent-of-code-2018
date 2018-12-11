extern crate euclid;
extern crate nalgebra as na;
use euclid::{Rect, Point2D, Size2D};

static SERIAL_INPUT: i32 = 5153;
type DMatrixi32 = na::Matrix<i32, na::Dynamic, na::Dynamic, na::MatrixVec<i32, na::Dynamic, na::Dynamic>>;

fn power(x: usize, y: usize) -> i32 {
    let x = x as i32;
    let y = y as i32;
    let rack_id = x + 10;
    (((rack_id * y + SERIAL_INPUT) * rack_id) % 1000) / 100 - 5
}

fn max_power(bbox: Rect<usize>) -> Point2D<usize> {
    let mut max_area = Point2D::new(0,0);
    let mut max_power = std::i32::MIN;
    for x in bbox.origin.x..bbox.origin.x + bbox.size.width - 2 {
        for y in bbox.origin.y..bbox.origin.y + bbox.size.height - 2 {
            let power = power(x,y) + power(x+1,y) + power(x+2,y) + power(x,y+1) + power(x+1,y+1) + power(x+2,y+1) + power(x,y+2) + power(x+1,y+2) + power(x+2,y+2);
            if power > max_power {
                max_area = Point2D::new(x, y);
                max_power = power;
            }

        }
    }

    max_area
}

fn max_power_naive(bbox: Rect<usize>) -> (Point2D<usize>, usize) {
    let mut max_area = (Point2D::new(0,0), 0);
    let mut max_power = std::i32::MIN;

    let grid = DMatrixi32::from_fn(bbox.size.width as usize, bbox.size.height as usize, |x, y| power(x + 1, y + 1));

    let mut memory = std::collections::HashMap::new();

    for size in 1..=bbox.size.width {
        println!("{}", size);
        if max_power > (size * size * 9) as i32 {
            break;
        }

        for x in bbox.origin.x..bbox.origin.x + bbox.size.width - (size - 1) {
            for y in bbox.origin.y..bbox.origin.y + bbox.size.height - (size - 1) {

                let power_below = *memory.get(&(x,y,size-1)).unwrap_or(&0);
                // Non-nalgebra version:
                //let power = power_below + (0..size).fold(0, |acc, dx| acc + power(x+dx, y+size-1)) + (0..size-1).fold(0, |acc, dy| acc + power(x+size-1, y+dy));

                let s1: i32 = grid.slice((x-bbox.origin.x, y+size-1-bbox.origin.y), (size, 1)).iter().sum();
                let s2: i32 = grid.slice((x+size-1-bbox.origin.x, y-bbox.origin.y), (1, size-1)).iter().sum();
                let power = power_below + s1 + s2;

                memory.insert((x,y,size), power);

                if power > max_power {
                    max_area = (Point2D::new(x, y), size);
                    max_power = power;
                }
            }
        }
    }

    max_area
}

/// Uses a summed-area table.
fn max_power_summed_area_table(bbox: Rect<usize>) -> (Point2D<usize>, usize) {
    let mut max_area = (Point2D::new(0,0), 0);
    let mut max_power = std::i32::MIN;

    let mut grid = DMatrixi32::from_element(bbox.size.width + 1, bbox.size.height + 1, 0);
    for x in 1..=bbox.size.width {
        for y in 1..=bbox.size.height {
            grid[(x,y)] = power(x,y) + grid[(x-1,y)] + grid[(x,y-1)] - grid[(x-1,y-1)]
        }
    }

    for size in 1..=bbox.size.width {
        println!("{}", size);
        for x in bbox.origin.x..bbox.origin.x + bbox.size.width - (size - 1) {
            for y in bbox.origin.y..bbox.origin.y + bbox.size.height - (size - 1) {
                let power = grid[(x-1,y-1)] + grid[(x+size-1,y+size-1)] - grid[(x+size-1,y-1)] - grid[(x-1,y+size-1)];
                if power > max_power {
                    max_area = (Point2D::new(x, y), size);
                    max_power = power;
                }
            }
        }
    }


    max_area
}

fn main() {
    println!("Part 1: {}", max_power(Rect::new(Point2D::new(1,1),Size2D::new(300,300))));
    println!("Part 2: {:?}", max_power_summed_area_table(Rect::new(Point2D::new(1,1),Size2D::new(300,300))));
}

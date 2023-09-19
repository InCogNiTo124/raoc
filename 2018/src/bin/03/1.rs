use std::{
    collections::HashSet,
    io::{self, Read},
};

use scanf::sscanf;

struct Rect {
    id: u32,
    left: u32,
    top: u32,
    bottom: u32,
    right: u32,
}

fn is_overlap(r1: &Rect, r2: &Rect) -> bool {
    r1.left < r2.right && r1.right > r2.left && r1.top < r2.bottom && r1.bottom > r2.top
}

fn get_intersection_area(r1: &Rect, r2: &Rect) -> Rect {
    let left = r1.left.max(r2.left);
    let right = r1.right.min(r2.right);
    let top = r1.top.max(r2.top);
    let bottom = r1.bottom.min(r2.bottom);
    Rect {
        id: 0,
        left,
        top,
        bottom,
        right,
    }
}

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let rectangles = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let mut id: u32 = 0;
            let mut left: u32 = 0;
            let mut top: u32 = 0;
            let mut width: u32 = 0;
            let mut height: u32 = 0;

            sscanf!(line, "#{} @ {},{}: {}x{}", id, left, top, width, height);
            Rect {
                id,
                left,
                top,
                right: left + width,
                bottom: top + height,
            }
        })
        .collect::<Vec<_>>();

    let mut points: HashSet<(u32, u32)> = HashSet::new();
    // println!("{}", rectangles.len());

    for i in (0..rectangles.len() - 1) {
        let r1 = &rectangles[i];
        for j in (i + 1)..rectangles.len() {
            let r2 = &rectangles[j];
            if is_overlap(r1, r2) {
                //println!("{} & {}", r1.id, r2.id);
                //println!("{}", get_intersection_area(r1, r2));
                let overlap = get_intersection_area(r1, r2);
                populate(&mut points, overlap);
            }
        }
    }
    println!("{}", points.len());
}

fn populate(points: &mut HashSet<(u32, u32)>, overlap: Rect) {
    for x in overlap.left..overlap.right {
        for y in overlap.top..overlap.bottom {
            points.insert((x, y));
        }
    }
}

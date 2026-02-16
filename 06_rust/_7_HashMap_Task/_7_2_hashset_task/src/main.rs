use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::mem;

#[derive(Clone, Debug, Eq , PartialEq , Hash)]
struct Point {
    x: i32,
    y: i32,
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut points: HashSet<Point> = HashSet::new();

    // try_reserve (safe allocation)
    if let Err(e) = points.try_reserve(5) {
        println!("Failed to allocate HashSet: {:?}", e);
        return Err(Box::new(e));
    }

    points.insert(Point { x: 5,  y: 1 });
    points.insert(Point { x: -3, y: 4 });
    points.insert(Point { x: 10, y: 2 });
    points.insert(Point { x: -7, y: 9 });
    points.insert(Point { x: 0,  y: 3 });

    // clone()
    let backup = points.clone();

    // retain() → keep only x >= 0
    points.retain(|p| p.x >= 0);

    // take() → move filtered data out
    let active_points = mem::take(&mut points);

    let mut final_points: HashSet<Point> = HashSet::new();

    // try_reserve for new set
    final_points.try_reserve(active_points.len())?;

    // extend()
    final_points.extend(active_points);

    println!("Backup (original data): {:?}", backup);
    println!("Final filtered points: {:?}", final_points);

    Ok(())
}

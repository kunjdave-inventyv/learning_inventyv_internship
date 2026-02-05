use std::collections::HashMap;
use std::mem;

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut points: HashMap<u32, Point> = HashMap::new();

    if let Err(e) = points.try_reserve(5) {
        println!("Failed to allocate HashMap: {:?}", e);
        return  Err(Box::new(e));
    }
    points.insert(1, Point { x: 5,  y: 1 });
    points.insert(2, Point { x: -3, y: 4 });
    points.insert(3, Point { x: 10, y: 2 });
    points.insert(4, Point { x: -7, y: 9 });
    points.insert(5, Point { x: 0,  y: 3 });

    let backup = points.clone();

    points.retain(|_, p| p.x >= 0);

    let active_points = mem::take(&mut points);

    let mut final_points: HashMap<u32, Point> = HashMap::new();

    final_points.try_reserve(active_points.len())?;

    final_points.extend(active_points);

    println!("Backup (original data): {:?}", backup);
    println!("Final filtered points: {:?}", final_points);

    Ok(())
}

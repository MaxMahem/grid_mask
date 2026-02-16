use grid_mask::num::GridPos;
use grid_mask::{GridMask, GridPoint};

fn main() {
    let t_shape = [(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)];
    let t_shape = t_shape.iter().map(|(x, y)| GridPoint::new(GridPos::new(*x).unwrap(), GridPos::new(*y).unwrap()));

    print!("Created coordinates: ");
    t_shape.clone().for_each(|p| print!("{}, ", p));
    let mask: GridMask = t_shape.collect();

    println!();
    // println!("Resulting mask visual:\n{}", mask.visualize('#', '.'));
}

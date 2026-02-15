use grid_mask::{ArrayGrid, ArrayVector};
use std::str::FromStr;

fn visualize<const W: u16, const H: u16, const WORDS: usize>(
    grid: &ArrayGrid<W, H, WORDS>,
    set: char,
    unset: char,
) -> String {
    let mut s = String::new();
    for (i, is_set) in grid.cells().enumerate() {
        if i > 0 && i % (W as usize) == 0 {
            s.push('\n');
        } else if i > 0 {
            s.push(' ');
        }
        s.push(if is_set { set } else { unset });
    }
    s
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pattern = "
        . . . . . . . . . .
        . # # # # # # # # .
        . # . . . . . . # .
        . # . # # # # . # .
        . # . # . . # . # .
        . # . # . . # . # .
        . # . # # # # . # .
        . # . . . . . . # .
        . # # # # # # # # .
        . . . . . . . . . .
    ";

    // 10x10 grid = 100 bits. 2 words (u64) are needed.
    println!("Original Grid Pattern (10x10):");
    println!("{}", pattern);

    // Create a 10x10 grid
    let mut grid: ArrayGrid<10, 10, 2> = ArrayGrid::from_str(pattern)?;

    println!("Parsed Grid:");
    println!("{}", visualize(&grid, '#', '.'));

    // Translate East by 3
    println!("\nTranslate East (3, 0):");
    println!("Notice the rightmost columns are shifted out and lost.");
    grid.translate(ArrayVector::new(3, 0));
    println!("{}", visualize(&grid, '#', '.'));

    // Reset grid
    grid = ArrayGrid::from_str(pattern)?;

    // Translate West by 3
    println!("\nTranslate West (-3, 0):");
    println!("Notice the leftmost columns are shifted out and lost.");
    grid.translate(ArrayVector::new(-3, 0));
    println!("{}", visualize(&grid, '#', '.'));

    // Reset grid
    grid = ArrayGrid::from_str(pattern)?;

    // Translate South by 3
    println!("\nTranslate South (0, 3):");
    println!("Notice the bottom rows are shifted out and lost.");
    grid.translate(ArrayVector::new(0, 3));
    println!("{}", visualize(&grid, '#', '.'));

    // Reset grid
    grid = ArrayGrid::from_str(pattern)?;

    // Translate North by 3
    println!("\nTranslate North (0, -3):");
    println!("Notice the top rows are shifted out and lost.");
    grid.translate(ArrayVector::new(0, -3));
    println!("{}", visualize(&grid, '#', '.'));

    Ok(())
}

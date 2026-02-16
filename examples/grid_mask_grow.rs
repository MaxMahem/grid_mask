use grid_mask::{Cardinal, GridMask};

fn main() {
    let crosses: GridMask = "
        . . . . . . . .
        . . # . . . . .
        . # # # . . . .
        . . # . . . . .
        . . . . . # . .
        . . . . # # # .
        . . . . . # . .
        . . . . . . . .
    "
    .parse()
    .expect("crosses should be valid");

    println!("Original mask (crosses):");
    // let crosses_visualized = crosses.visualize('#', '.');
    // println!("{crosses_visualized}");

    let grown_crosses = crosses.grow::<Cardinal>();

    println!("Grown mask:");
    // println!("{grown}", grown = grown_crosses.visualize('#', '.'));

    println!("Target mask (diamonds):");

    let diamonds: GridMask = "
        . . # . . . . .
        . # # # . . . .
        # # # # # . . .
        . # # # . # . .
        . . # . # # # .
        . . . # # # # #
        . . . . # # # .
        . . . . . # . .
    "
    .parse()
    .expect("diamonds should be valid");

    // println!("{}", diamonds.visualize('#', '.'));

    assert_eq!(grown_crosses, diamonds, "crosses should grow to diamonds");
    println!("Assertion passed: grown crosses match diamonds.");
}

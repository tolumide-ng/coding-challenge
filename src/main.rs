pub mod dynamic_programming;

// use dynamic_programming::can_sum::can_sum;
use dynamic_programming::how_sum::how_sum;

fn main() {
    println!(
        "Hello, world! {:?}",
        // how_sum::recursive_how_sum(7, vec![2, 3])
        how_sum::recursive_how_sum(7, vec![5, 3, 4, 7]).unwrap()
    );
}

pub mod dynamic_programming;

// use dynamic_programming::can_sum::can_sum;
// use dynamic_programming::how_sum::how_sum;
use dynamic_programming::best_sum::best_sum;

fn main() {
    println!(
        "Hello, world! {:?}",
        best_sum::recursive_best_sum(7, vec![5, 3, 4, 7])
    );
}

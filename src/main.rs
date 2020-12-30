pub mod dynamic_programming;

// use dynamic_programming::can_sum::can_sum;
use dynamic_programming::how_sum::how_sum;

fn main() {
    println!(
        "Hello, world! {:?}",
        // how_sum::recursive_how_sum(7, vec![2, 3])
        how_sum::memoized_how_sum(300, vec![14, 7])
    );
}

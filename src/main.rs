pub mod dynamic_programming;

use dynamic_programming::can_sum::can_sum;

fn main() {
    println!(
        "Hello, world! {:?}",
        can_sum::memoized_can_sum(300, vec![7, 14])
    );
}

pub mod dynamic_programming;

// use dynamic_programming::can_sum::can_sum;
// use dynamic_programming::how_sum::how_sum;
// use dynamic_programming::best_sum::best_sum;
use dynamic_programming::how_sum::how_sum::tabulated_how_sum;

fn main() {
    println!(
        "Hello, world! {:?}",
        // best_sum::memoized_best_sum(100, vec![1, 2, 5, 25, 30])
        // best_sum::memoized_best_sum(100, vec![1, 2, 5, 25, 30])
        // recursive_all_construct("abcdef", vec!["ab", "abc", "cd", "def", "abcd", "ef", "c"]),
        tabulated_how_sum(7, vec![5, 3, 4, 7])
    );
}

mod dynamic_programming;

// pub use crate::dynamic_programming::fibonacci;
// pub use crate::dynamic_programming::grid_traveller::grid_traveller;
pub use crate::dynamic_programming::can_sum::can_sum;

fn main() {
    println!(
        "WINDELLIN ---->>>>> {:#?}",
        can_sum::recursive_can_sum(7, vec![2, 3])
    );
}

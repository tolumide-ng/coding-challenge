mod dynamic_programming;

// pub use crate::dynamic_programming::fibonacci;
pub use crate::dynamic_programming::grid_traveller::grid_traveller;

fn main() {
    println!(
        "WINDELLIN ---->>>>> {:#?}",
        grid_traveller::memoized_grid_traveller(5, 3)
    );
}

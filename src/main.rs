// pub mod fib;
mod dynamic_programming;

pub use crate::dynamic_programming::fibonacci;

// use dynamic_programming::fib;

fn main() {
    println!("fahint talking <>>>>>>{:?}", fibonacci::fib(6));
    println!("fahint talking <>>>>>>{:?}", fibonacci::fib(7));
    println!("fahint talking <>>>>>>{:?}", fibonacci::fib(8));
    // println!("fahint talking <>>>>>>{:?}", fibonacci::fib(50));
    println!("fahint talking <>>>>>>{:?}", fibonacci::memoized_fib(50));
}

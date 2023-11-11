// Usings (create shortcut for modules)
use basic_examples::guessing_game;
// Use with nested path
use basic_examples::{types_int, multi_dimensional_array};
// ...

// Module declaration (is accessible from main function - same scope)
mod basic_examples;

// Main entry point
// - can have parameters etc.
pub(crate) fn main() {
    println!("Starting modules ...");
    
    // Call modules function 
    guessing_game::play();
    types_int::wrapping_test();
    multi_dimensional_array::test();
    
    basic_examples::functions::test();
    basic_examples::control_flow::test();
    basic_examples::references::test();
    basic_examples::slices_test::test();
    basic_examples::structs::test();
    basic_examples::enums::test();
    basic_examples::vector::test();

    println!("Finished ..");
}

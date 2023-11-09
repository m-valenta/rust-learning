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

    println!("Finished ..");
}

mod parent {

    fn run_module(){
        
        // relative path using module identifier
        child_1::test_1();

        // relative path using self
        self::child_1::test_1();
    }

    mod child_1 {
      pub fn test_1() {}
    }
    mod child_2 {
      pub fn test_2() {

        // relative path using super
        super::child_1::test_1();

        // child_1 is inaccessible by this way (could be imported by use)
        // child_1::test_1(); 
      }
    }
}
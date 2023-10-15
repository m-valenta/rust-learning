// import other file/module
mod guessing_game;
mod types_int;
mod multi_dimensional_array;
mod functions;
mod control_flow;
mod references;
mod slices_test;
mod structs;
mod enums;

// Main entry point
// - can have parameters etc.
pub(crate) fn main() {
    println!("Starting modules ...");
    
    // Call modules function 
    guessing_game::play();
    types_int::wrapping_test();
    multi_dimensional_array::test();
    functions::test();
    control_flow::test();
    references::test();
    slices_test::test();
    structs::test();
    enums::test();

    println!("Finished ..");
}

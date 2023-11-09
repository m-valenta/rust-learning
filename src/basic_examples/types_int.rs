// Wrapping == checking of numeric overflow
// pub must be used to enable this file (module) in main.rs

pub fn wrapping_test() {
    println!("== int wrapping ==");

    let test: u8 = 255;

    // add with explicit wrapping of overflowed value
    let test = test.wrapping_add(2);
    println!("Wrapping - {test}");

    let test: u8 = 255;

    // add with explicit checking of overflowed value
    let test: u8 = match test.checked_add(2) {
        Some(result) => result,
        None => 0,
    };
    println!("Checking - {test}");

    let test: u8 = 255;

    // add with explicit overflowing test
    let (test, is_overflowed) = test.overflowing_add(2);
    println!("Overflowing - result: {test}, overflowing: {is_overflowed}");
}

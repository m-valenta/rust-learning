// Example form: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

// usings
//use rand::Rng; // external dependency (in .toml), introducing Rng "trait" into scope
use std::cmp::Ordering;
use std::io;

use rand::Rng;

pub fn play() {
    // println! is a macro that prints a string to the screen
    println!("Guess the number!");

    // using module/lib rand to generate rendom num in range 1..10
    // thread_rng() get random generator which is owned by execution thread and seeded by system
    // gen_range(..) method is defined in Rng trait
    // ..=10 👉 range epression which is taking number from interval <1,10>

    // cargo providing documentation 👉 cargo doc --open
    let secret_number = rand::thread_rng().gen_range(1..=10); // define immutable int

    // infinite loop
    loop {
        // Declaration var of type string (mutable)
        // :: Operator accesing member of type (same meaning as static - here for ctor)
        // let mut mutStr = "initial value"; working too

        let mut guess = String::new();
        
        println!("Please input your guess.");

        // Get user input
        // & reference - by default is immuttable
        // &mut reference - by default is not muttable

        // Read line returns: Result<T> = result::Result<T, Error> -> "enum" type for handling multiples states (Ok, Err)
        // state is called variants

        io::stdin() // calling stdin function from io module return instance of Stdin
            .read_line(&mut guess) // we should provide string var as a target for user input
            .expect("Failed to read line"); // This method handle error state (Err()) of result and crash program with defined message

        let trimmed_guess = guess.trim();
        
        // string placeholders
        // string with placeholders wariants marked with {}
        // placeholder can't hold expresion (only variable) - we can add them i positional fashion

        // let x = 1;
        // let y = 0;
        // println!("x = {x}, y = {y} and y + 2 = {}, {{}}, x + 150 = {}", y + 2, x + 150);
        println!("You guessed: {trimmed_guess}");

        // Match statements

        // Statement with return
        // Again working Result<T, E> - handling all branches this time (as a opposit to expect, whitch fail process)

        let guess: u32 = match trimmed_guess.parse() { // using variable shadowing (same name with multiple types)
            Ok(num) => num,
            Err(_) => { // batch with block of code, _ hides variable
                println!("Can not be processed, pls insert number!");
                continue; // loop control 
            }
        };

        // cmp call returns Ordering enum - this has the almoust same behaviour like the swich in c#
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break; // loop control
            }
        }
    }
}

// crate = modul/library
// link to external dependencie should be added into Cargo.toml
// random libray 👉 rand = "0.8.5"
// Dependencies are loaded from Crate.io 👉 something like nuget.org

// cargo build 👉 installing all dependenciess and mark version to Cargo.lock file to enamble consistent build
// If we need to update .lock file we can use 👉 cargo update
// update will still respect sem. ver. patterns from .toml file


/*
Enum type can define it's variants as a functions

Callee will return T (ok value), or E (error) and match expression automatically match them to correct variants
and call function 

pub enum Result<T, E>  
{
    Ok(T), 👉 takes succesfull value
    Err(E), 👉 takes error value (simillar to exceptions)
}

*/
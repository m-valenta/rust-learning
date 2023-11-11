# ⚙️Rust concepts [doc](https://doc.rust-lang.org/book/)

## Variables and Mutability

### Variables
Variables are _immutable by default_. Mutability is set by keyword _mut_. Name convention consists from lower case letters and underscores.

Example:
```
let x = 5;  
let mut y = 6;
```

### Constants
There is few difference. 
- Implicitly always immutable 
- May be set only to a constant expression [details](https://doc.rust-lang.org/stable/reference/const_eval.html)
- Can be declared at any scope (global, function, etc.)
- Must have type annotation
- _Name convention -> upper case letter + underscore_

Example:
```
const HOURS_IN_SECONDS: u32 = 60 * 60;
```

### Shadowing
Variable can be shadowed eq. can be defined multiple time. It means that variable is defined until is shadowed or it's scope ends.  
- Immutability of variable or it's type is given by actively defined variable.
- we can shadow both variable's value and type
Example of valid code:
```
    let x = 5;

    // value of x = 5

    {
        let mut x = x;
        x = x * 2;

        // value of x = 10

        println!("The value of x in the inner scope is: {x}");

        let x = "test str";

        // value of x = "test str"
    }
    
    // value of x = 5
    println!("The value of x is: {x}"); // 6
```

## Data Types
- statically typed -> each variable must have it's type (scalar/compound) at compile time 
- compiler can guess some values but sometime needs an annotation
- annotation format: `let {var_name}: {type}`
```
// shouldn't compile (because multiple numeric types can be used)
let guess = "48".parse().expect("Should be a number");

// With type annotation it's working correctly
let guess: u32 = "48".parse().expect("Should be a number");
```
## Scalar types

### Numeric type
- Integers
  - signed   i{8-128}
  - unsigned u{8-128} 
  - size isize / usize is given by underlaying architecture (x32, x64, ?)
  - literals
    - _Decimal_	           `98_222`
    - _Hex_	               `0xff`
    - _Octal_	           `0o77`
    - _Binary_	           `0b1111_0000`
    - _Byte_ (only for u8)  `b'A'`
  - Overflow
    - in `--debug` mode compilation, program panics on overflow 
    - in `--release` mode compilation, program doesn`t panic but _wrapping_ occurs
    ```
    // Wrapping example
    let test: u8 = 257; // test == 1
    ``` 
    - _implicit relaying on wrapping_ can be considered as a _mistake_. In rust exists _explicit variant_ how to handle wrapping
      - `{variable}.wrapping_{operation}({params})` perform operation with _enabled wrapping_ in all modes
      - `{variable}.checked_{operation}({params})` return _enumeration_ with result or none in all modes
      - `{variable}.overflowing_{operation}({params})` return _wrapped result_ and boolean tuple which _indicates overflow_
      ```
      // Example
      let t: u8 = 255;
      
      // t value will be 1
      let t: u8 = t.wrapping_add(2);   
      ```
- Floating-Point types
  - for number with decimal point
  - all floating-point are _always signed_
  - default type is `f64` and there is `f32` too.

### Other types
- `bool`
- `char`
  - `let c = 'T'`
  - 4 bytes / unicode scalar value

## Primitive compound types

### Tuple
```
let tup-with-types: (i32, f64, u8) = (500, 6.4, 1);
let tup = (500, 6.4, 1);
```
Tuple allow _destructuring_ it's values
```
let (x, y, z) = (500, 6.4, 1);
```
Values in tuple can be accessed by it's index (starting from zero)
```
let tup = (500, 6.4, 1);
let secondValue = tup.1;  //6.4
```
### Array
```
let a1 = [1, 2, 3];
let a2: [i32] = [1, 2, 3]; // With type
let a3: [i32; 3] = [1, 2, 3]; // With type and size
```

Initialization
```
let a = ["a", 3]; // ["a", "a", "a"]
```

Access (indexed from zero)
```
let a = [1, 2, 3];
let first = a[0];
```

Mutable array
```
let mut a = [1, 2, 3];
a[0] = 3;
```

Multidimensional arrays
```
    let a: [[i32; 2]; 2] = [
        [0, 1],
        [2, 3]
    ];

    // Using initialization
    let b = [[1;2];2];

    // indexing
    let lastItem = [1][1];
```

Array length `{array_name}.len()`
```
    // Single dimensional
    let a = [1, 2];
    let length = a.len();

    // Multi dimensional
    let b = [[1;2];3];
    let height = b.len(); // 3
    let width = b[0].len(); // 2
```

## Functions
Declaration:
```
fn {name}([arg_list]) -> {ret_type} {
}
```
- declaration can be placed every where, there is no need to keep some order
- `arg_list` must contains all argument's names and types
- how to return value (two options)
   - `return {value};`
   - last line contains _expression_ (without `;`)
- example:
```
fn test(p1: i32, p2: i32) -> i32 {
  if p1 > 5 {
    return 5 * p2;
  }

  // block with last line == expression is expression
  // expression is returned as the function result    
  {
    let x = p1 * p1;
    x * p2 // expression
  }
}
``` 
- how to call function: `let x = test(1, 2);`

## Comments
- Only one type: `// -> comment to the end of line`
- Documentation comments use `///`

## Control flow

### if
- expression (can be used on right side of assignment)
- exapmle:
```
if {confition} {
 ...
}
else if {condition} {
 ...
}
else {
 ...
}
```
### loops
- expressions (can be used on right side of assignment)
- can be controlled by
  - break (more variants)
    - `break;` - break loop in closest scopes 
    - `break {value};` - break loop in closest scope and return value 
    - `break '{label_name};` -break labeled loop (nesting)
  - continue
- can be labeled by: `'{label_name}: {loop_type} { ... }`
- types:
  - `loop { ... }` - infinit loop
  - `while {condition} { ... }` - while in common sense
  - `for {item_name} in {collection} { ... }` - foreach like cycle
  - `for {value_name} in (0..10) {...}` - common for (from...to) using _Range_

## Ownership 
- is set of static (compile-time) rules, which are used for managing memory on the _heap_
- application of these rules doesn't slow down an application during runtime
- rules:
  - Each value in Rust has an owner.
  - There can only be one owner at a time.
  - When the owner goes out of scope, the value will be dropped.
- Rust using Resource _Acquisition Is Initialization_(_RAII_) pattern.
  - when _heap_ value goes out of scope, rust automatically calls it's special `.drop()` function (probably triggered internaly by `}` ).

- Special traits
  - _Drop trait_ let us implement custom `drop()` function, only for value stored on _heap_
  - _Copy traits_ let us mark the type to be "copy" during assignment, valid only for type stored on stack with all parts stored on stack as well
  ```
    // Implement the Copy and Clone traits
    #[derive(Copy, Clone)] 
    struct MyType {
        data: u32,
    }

        let original = MyType { data: 42 };
        let copied = original; // This line creates a bitwise copy of the original MyType instance
  ```
- Example of moving of ownership: 
  ```
    // stack (value)type
    let x = 5;
    let y = x; // value is copied, both variables are valid

    // vs heap (reference)type

    let s1 = String::from("hello");
    let s2 = s1; // ownership is moved, s1 is no more valid from this point!
  ```
  - Example function and ownership
  ```
    fn takes_ownership(s: String) {
      // This function takes owner ship
    }

    fn gives_ownership() -> String {
      // This function return ownership to caller side
      String::from("hello")
    }

    let s3 = String::from("hello");
    takes_ownership(s3) // ownership is moved, s3 is no more valid

    let s4 = gives_ownership(); // s4 became owner of String value; 
  ```

  - Example of "dangling" (not valid reference)
  ```
  // ERROR dangle returns a reference to a String
  fn dangle() -> &String { 

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
  } // Here, s goes out of scope, and is dropped.

  // Valid variant - ownership moves to caller
  fn no_dangle() -> String {
    let s = String::from("hello");

    s // we pass ownership of String out from the function
  } 

  ```

## References (and borrowing)
- reference is declared by: `&{type}`
  - multiple readonly references of single value in same scope is allowed without any restriction 
- mutable reference is declared by: `&mut {type}`
  - mutable value can have _only one mutable reference in given scope_
  - combination of mutable and readonly reference in same scope is prohibited too

- example of references:
```
  // values
  let s1 = String::from("hello");
  let mut s2 = String::from("hello");

  // how to define
  let s1_ref = &s1;
  let s1_ref_2 = &s1; // multiple readonly refs is allowed
  let s2_mut_ref = &mut s2;
  // let s2_ref = &s2; readonly ref can be used for mutable value too, but not in this scope (because mutable ref is already defined)

  // how to declare
  fn(s1: &String, s2: &mut String) {
    ...
  }

```
- example of "dangled" (invalid) reference protection

```
// Error (compile-time)
fn dangle() -> &String {
    let s = String::from("hello"); // s is a new String

    &s // returns a reference
} // Here, s goes out of scope, and is dropped.


// Correct 
fn no_dangle() -> String {
  String::from("hello") // return value and it's ownership back to caller
} 
```

## Slice (types)
- slice is reference of some range from collection or string
  - readonly: `let slice = &{variable}[{from}..{to}];`
  - mutable: `let mut slice = &mut {variable}[{from}..{to}]`;  
- (mutable) collection slice can be marked as mutable
- String slice has &str type (is readonly)
- ranges (intervals includes from/to)
  - `[{from}..{to}]` 
  - `[{from}..]` from to end
  - `[..{to}]` from start to
  - `[..]` whole value
- example:
```
let s = "string literal";
let str = String::from("string");
let mut col = [1, 2, 3];

// Readonly
let s_slice: &str = &s[1..2]; // "tr"
let s_tr: &str = &str[2..3] // "ri"

// Mutable slice of collection
let s_col: &mut [i32] = &mut col[..1]; // [1, 2]
s_col[0] = 154;
```

## Structs

### _struct_ definition
```

#[derive(Debug)]  // Trait - enable struct to in combination with {:?}, or {:#?} in print!/printl!, or in dbg!
struct User {
  active: bool,
  name: String,
  req_count: u32
}
```

### _tuple struct_ definition 
- useful when:
 - names of fields make no sense 
 - it is meaningful to typed "common" tuple (enforce semantic of method's arguments)
```
struct Color(u8, u8, u8, f64);
```

### _unit-like struct_ definition 
 - has only name and no fields
 - useful for traits
```
struct SomeType;
``` 

### Instantiating
- basic (we should specify all fields)
``` 
 // structs
 let user = User {
    active: true,
    name: String::from("Karl"),
    req_count 124 
 };
 let mut user_mut = User {
    active: true,
    name: String::from("Jon"),
    req_count 124 
 };

 // tuple struct
 let black = Color(0, 0, 0, 1.0);
 let mut black_mut = Color(0, 0, 0, 1.0);

 // unit-like
 let t_inst = SomeType;

```
- Field init shorthand (js like shortcut fot matching names)
```
let active = true;
let req_cnt = 153;

// We don't have to duplicate same names
let user3 = User {
  active, 
  req_cnt,
  name: String::from("Bob")
};
```
- Struct Update Syntax (from other struct)
```
  // Copy all fields
  // warning ownership of name field was moved from user3 to user4 
  let user4 = User{..user3};
```

 ### field access
 ```
 // structs
 user_mut.req_cnt = user.req_cnt;

 // tuple like  
 black_mut.3 = back.3;
 let Color(r, g, b, a) = black;
 let Color(r_m, g_m, b_m, mut a_m) = black_mut;
 a_m = 0.5; // value have been moved -> no change in black_mut 

 ```

 ### Reference handling (lifetimes)
 - to be done later

### Methods
- methods are defined in impl block `impl {type} { ... }`
- multiple `impl` blocks _are allowed_
- method has `self`, `&self`, `&mut self` as _first parameter_
  - `self` can be used in some special cases borrow reference (object can not be used after call)
- method can have same name as a field (can implement _getter_ in that way)
- method call: `inst.method([parameters]);`
- method calls using automatic dereference: `(&inst).method([params]);`
- _associated function_ 
  - function in `impl` block without `self` parameter 
  - same as _static_ (are accessible from struct namespace), 
  - can implement _constructor_
  - call: `{struct_name}::method(..);`
```
impl Color {
  // ctor
  fn black() -> Color {
    Color(0, 0, 0, 1.0)
  }

  fn get_alpha(&self) -> f64 {
    self.3
  }

  fn set_alpha(&mut self, a: f64) {
    self.3 = a;
  }
}
```

## Enums
Combine enum in common sense known from other languages and _"binding"_ to enum values.

### Definition
Any type can be bind to enum values - it includes structs and other enums. 
```
enum Message {
    Quit,   // Common value 
    Move { x: i32, y: i32 }, // Bind value with named fields  
    Write(String), // Bind (single) value
    ChangeColor(i32, i32, i32), // Bind multiple values
}
```

### Instantiating
Values ar defined under enum (name) namespace, so we can use `::` operator.

```
let message_1 = Message::Quit;
let mut message_2 = Message::Move({x: 1, y: 2}); 
```

### Method
Like `structs` enum can have methods defined in similar way.

```
impl Message {
  // Into self is bind value of Message instance 
  execute(&self) {
    // do something
  }
}
```

Methods can be called like this (using `.`)
```
Message::Write(string::from("Test")).execute();

```

### match (operator)
Similar to switch operator in other languages. It's syntax consists from
- `match` keyword and 
- multiple branches divided by `,`. 
Each branch contains of
- pattern  
- `=>` operator and
- expression witch will be executed / returned if pattern match.

All possible values  must be covered by some branch. We can use _default branch_ for the rest of values:
  - `other => {expression}` expression can use original value which is bind to _value_ variable
  - `_ => {expression}`  same as previous but we don!t care about original value

All bind values are passed by ref, This is because Rust uses pattern matching to destructure values, and destructuring a value requires a reference to that value.
- rust contains `*` _dereference operator_ to get original value
- value is dereferenced automatically when we used it in expression

Example:
```
impl Message{
  fn execute(&self) {
    match self {
      Message::Quit => println!("End"),
      Message::ChangeColor(r, g, b) => {
        // Block expression can be use for multi-line branches
        
        println!("Red {:?}", r);
        println!("Green {:?}", g);
        println!("Blue {:?}", b)
      }, // , is optional here
      Message:Write(text) => println!("Write {}", text)
      Message:Move {x, y} => println!("Move to (x: {:?}, y: {:?})", x, y);
      _ => println!("Others")
    }
  }
}
```

### if let 
- syntactic sugar/shortcut for `match`.
- syntax: `if let {pattern} = {enum_value} { some code }`
- can contain `else` block as regular `if` 
- Example:
```
if let Message:Write(text) = message_inst {
  println!("Write: {}", text);
} else {
  println!("Unsupported");
}
```

### Option<T>
- enum defined in standard library and included (with it's values) in prelude (automatically used parts of std. lib.).
- It is used in lang design to represent _null / undefined_ values
- Definition looks like this:
```
Option<T> {
  None,
  Some(T)
}
```
- Example of usage:
```
fn safe_divide(dividend: f64, divisor: f64) => Option<f64> {
  if divisor == 0
    None
  
  Some(dividend / divisor)
}
```

## Project structuring
Rust offer several levels of hierarchy
- _Cargo workspace_ is used for large projects. It consist from multiple packages (which evolve together)
- _Package_ consists of at least one crate
- _Crate_ is minimal unit which can be compiled. It consist from modules hierarchy.

### Crate
Is minimal compilable unit. It keeps modules hierarchy.

It has two forms:
- Binary crate
  - it is compiled into executable
  - `src` folder must contain `main.rs` file with entry point (main function) 
- Library
  - library / dll in common sense
  - `src` folder must contain `lib.rs` file

`lib/main.rs` file are used as entry-point/root for module hierarchy

### Package
Is bundle of one or more crates which provides some functionality. Package is allowed to have just _single library crate_ (should hold common logic) and multiple _binary crates_ (executables). 
- It is created by `cargo new {project_name}`
- structure:
   - `Cargo.toml` file, which describe how to build crates
   - `src` folder
     - `lib.rs` / `main.rs` are _crates roots_ and are automatically considered to define binary/library crate which has _same name as a package_
     - module files
     - submodule folders
     - `src/bin` directory which is used for defining _other binary_ crates. All file placed here can be considered as new crate definition (with same name as file has). 

### Modules
- module hierarchy starts in `main.rs/lib.rs`.
  - Modules can be defined in root file like this
```
mod {module_name} {
  // module content can be declared directly here
}
```
 - Modules can be extracted to separated files
```
mod {module_name};
```
   - it references `src\{module_name}.rs` file
     - which can define _submodules_ by using `mod {submodule_name};`
       - submodule file must be placed into `src\{module_name}\{submodule_name}.rs` 

### paths
Paths are used for accessing defined modules content. Path has two variants
- absolute: `crate::parent::child1::test1()` starting by literal `crate` and modules (declarations) are separated by `::`
- relative: starts from given scope and can use _module name_ or `self`/`super` literals
  - `super` is similar to `..\` operator in file system - it moves scope to parent module
  - `self` can be useful when we need distinguish between items in the current module and items in the parent scope   

Relative paths example
```
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
```

### pub modifier
modules and other file's content is consider to be private. It is accessible only for _sibling and child_ definitions. We can use `pub` modifier to make definitions accessible from other scopes.

Example:
```
// from outside we can access only root::test function
pub mod root {
    pub fn test(param: &str){
        println!(param);

        // internal_test is accessible because it is in the same module ( == sibling) 
        // run function is accessible because it is marked as public 
        internal_test::run();

        // internal_test.run_2() is inaccessible
    }

    mod internal_test{
        pub fn run(){
            // Test is accessible because it is defined in the parent module (it is child)
            test("from internal test");
        }

        fn run_2 {}
    }
}
```

Special behavior of `pub` modifier for:
- _struct_ when marked as pub - it has no effect on methods and fields. They must be marked by pub to be accessible. 
- _enum_  when marked as pub all it's variants star to be accessible          

### use keyword
Similar purpose as using/import introduce path to scope. Format is `use {path};`. Take effect only for wrapping scope.

There are few specifics:
- _external package_ - after adding reference to Cargo.toml we can use `use {package_name}::{path}`
- _idiomatic_ - import single func, struct enum etc. into current scope
- `as` operator can rename imported types if name collide etc. `use {path} as {new_name};`
- `pub use {path};` can be used for _re-export_ imported types
- _nested paths_ can short use list `use path::{[sub_path_list]}`. 
example:
```
// example 1
use std::cmp::Ordering;
use std::io;

// can be simplified
use std::{cmp::Ordering, io};

// example 2
use std::io;
use std::io::Write;

// can be simplified by using self
use std::io::{self, Write};
```
- glob operator `use {path}::*` for bring all public items into scope

## Collection
Basic collection are Vector, String, hash map. More collection can be found in [doc](https://doc.rust-lang.org/std/collections/index.html).

### Vector ([doc](https://doc.rust-lang.org/std/vec/struct.Vec.html)) 
List of value of the same type `Vec<T>`.
- Vector is owner of it's values, whe is dropped from the scope all values are dropped too
- Creation of new vector:
  - Immutable: `let values: Vec<i32> = Vec::new();`
  - Mutable: `let mut mut_values: Vec<i32> = Vec::new();`
  - From array `let mut mut_values = Vec::from([1, 2, 3, 4]);`
  - With initialization using macro: 
    - `let mut mut_values = vec![1, 2, 3];`
    - `let mut mut_values = vec![0, 2]; // [0, 0]`
- Adding new values:
  - `mut_values.push(4);`
  - `mut_values.extend([5, 6, 7]);`
  - `mut_values.insert(1, 2); // (index, value)`
- Cleaning
  - `mut.values.clean(); // remove all`
  - `mut.values.drain(0..5) // remove range and return values as enumerator`
  - `mut.values.remove(1);`
- Getting values
  - `let value = mut_values.pop();` 
    - returns `Option<T>`   
  - Indexer:
    - can cause program to panic
    - immutable: `let value = &values[10];`
    - mutable: `let second = &mut values[1];`
  - Creating slice (mut/immutable) from `Vec<T>`
      - `let slice: &[isize] = &v;`
      - `let slice: &mut [isize] = &mut v[2..];`
      - handy to use it as argument
  - `let value = mut_values.get(10);`
    - returns `Option<T>` it doesn't panic
    - mutable variant `.get_mut(10)`
  - Accessing value by dereference `*value`    
- Length
  - `mut_values.len;`
- Iterators
  - can be mutable (&)/immutable (&mut)
  ```
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // * operator dereferences value in i 
        *i += 50;
    }
  ```

  ### String ([doc](https://doc.rust-lang.org/std/string/struct.String.html#))
  The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

  Two representations 
    - `String`
    - string slice
     - `&str`
     - `&mut str`
  
  In rust the string is very similar (eq. wrapper) to `Vec<u8>` with some limitations and special api.
  - creation
    - `String::new()`
    - `String::from("")`
    - `32.to_string() // has any type implementing display trait`
  - updating (`let mut s = String::new();`)
    - append string `s.push_str("new string");`
    - append char `s.push('l');`
    - Using `+` operator
      - internally using `String.add(self, other: &str) -> String`
    ```
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    
    // After this line s1 cannot be used (owner ship moved)
    let s3 = s1 + &s2; // + &s4 ... + &sn; 
    ```
    - using `format!` macro
    ```
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    ```
    - _Indexing_ (eq. getting char by index) is _not working_ due internal representation (visible character can be represent by multiple bytes)
    - instead of indexing we can use _slicing_. Rust disallow result to not to be valid string
    ```
    // 1 char == 2 bytes
    let hello = "Здравствуйте";
    
    // Valid
    let s = &hello[0..4];

    // Invalid (cause panic)
    let s = &hello[0..1];
    ```
    - Iterating over string. We have to specify what representation we want to iterate.
      - bytes (numbers): `for b in "Зд".bytes(){...}`
      - chars (scalars): `for b in "Зд".chars(){...}` 
    

Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: 
- as bytes, 
- scalar values, 
- and grapheme clusters (the closest thing to what we would call letters).
```
// Original value
“नमस्ते”

// Bytes
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]

// Scalars
// '्', 'े' are nor letters but diacritics
['न', 'म', 'स', '्', 'त', 'े']

// Grapheme clusters
["न", "म", "स्", "ते"]
```

## Hash maps ([doc](https://doc.rust-lang.org/std/collections/struct.HashMap.html))
The type `HashMap<K, V>`` stores a mapping of keys of type K to values of type V using a hashing function.
- hashing function 
  - _SipHash_ that can provide resistance to Denial of Service (DoS) attacks involving hash tables1.
  - Not the fastest hashing algorithm available, but the trade-off for better security.
  - you can switch to another function by specifying a different `hasher` (type that implements the `BuildHasher` trait).
- Creation
   - require std lib usage `use std::collections::HashMap;`
   - `let mut scores = HashMap::new();`
- Add values
  - `scores.insert(String::from("Blue"), 10);`
    - it overwrites existing values
    - inserting takes ownership over inserted values
  - `score.entry("something").or_insert(0);`
    - insert value only if not exist 
    - `entry(...)` method returns `Entry` enum
    - `or_insert(...)` method _returns a mutable reference_ (`&mut V`) to the value for the specified key (can be used by `*` operator).
- Accessing values
  - `let score = scores.get(&team_name);`
  - returns `Option<T>`
- Iterating
```
for (key, value) in &scores {
    println!("{key}: {value}");
}
```
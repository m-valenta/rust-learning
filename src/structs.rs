#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    req_count: u32,
}

#[derive(Debug)]
struct Color(u8, u8, u8, f64);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Constructor like / static method (missing self reference)
    // accessible by Rectangle::square(...)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // Method are function with self reference (must be first parameter)
    fn area(&self) -> u32 { // &self 👉 immutable reference to self
        self.width * self.height
    }

    fn grow(&mut self, width: u32, height: u32) { // &mut self 👉 mutable reference to self
        self.width += width;
        self.height += height;
    }

    // Rarely used method, which take ownership of self
    fn area_and_finalize(self) -> u32 { // self 👉 taking ownership of self, self is not valid after this method
        self.width * self.height
    }
}

pub fn test() {
    let mut user1 = User {
        active: true,
        name: String::from("John"),
        req_count: 0,
    };
    user1.active = false;
    user1.req_count += 1;

    // cannot use user1 after this line (is borrowed to copy)
    let user2 = copy_user(user1);

    println!("Structs test1 - user2: {:?}", user2);

    let user3 = User {
        name: user2.name.clone(), // protect name (ownership) from moving
        ..user2
    };

    println!("Structs test2 - user2: {:?}", user2);
    println!("Structs test2 - user3: {:?}", user3);

    let color = Color(255, 0, 0, 1.0);
    let Color(_r, _g, _b, mut a) = color; // destructuring with mutable (values are moved/copied)
    a = dbg!(a + 0.75); // dbg! macro is used for debugging (print value and return it)

    println!("Structs test4 - color: {:#?}, a: {:?}", color, a); // Other format {:#?} 👉 pretty print
    
    dbg!(&color); // debug macro can use ref (use without move / taking ownership)
    dbg!(color);  // debug macro move / taking ownership, but we can use return value 


    let mut rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    rect1.grow(10, 10);

    println!("Structs test5 - rect1: {:?} with area: {:?}, squre: {:?}", rect1, rect1.area(), Rectangle::square(10));
    
    // after this line rect1 is not valid (moved to method)
    println!("Structs test5 - rect1: {:?}", rect1.area_and_finalize());

}

fn copy_user(user: User) -> User {
    User {
        name: user.name.clone(),
        ..user // can not use &User
    }
}
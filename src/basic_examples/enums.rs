use rand::Rng;

pub fn test() {
    let message = Message::Move { x: 10, y: 20 };
    if let MessageResult::Success = message.call() {
        println!("Move succeeded!");
    } else {
        println!("Move failed!");
        return;
    }

    let message = Message::Write(String::from("Hello world!"));
    if let Message::Write(text) = message {
        println!("Write succeeded: {}", text);
    } else {
        println!("Write failed!");
        return;
    }

    let message = Message::ChangeColor(255, 0, 0);
    if let MessageResult::Success = message.call() {
        println!("Change color succeeded!");
    } else {
        println!("Change color failed!");
        return;
    }

    let message = Message::Quit;
    if let MessageResult::Success = message.call() {
        println!("Quit succeeded!");
    } else {
        println!("Quit failed - continue");
    }

    let rand_divider = rand::thread_rng().gen_range(0..=10);
    if let Some(result) = safe_division(10, rand_divider) {
        println!("Division succeeded: 10/{:?} = {:?}", rand_divider, result);
    } else {
        println!("Division failed - divider: {:?}!", rand_divider);
    }

}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum MessageResult {
    Success,
    Error(String),
}

impl Message {
    fn call(&self) -> MessageResult {
        match self { // sel
            // all bind values are passed by ref. due to pattern matching
            Message::Quit => MessageResult::Error(String::from("Quit is not implemented")),
            Message::Move { x, y } => Message::move_to(*x , *y ), // parameters are not expressions we should dereference them
            Message::Write(text) => Message::write(text),
            Message::ChangeColor(r, g, b) => Message::change_color(*r, *g, *b),
        }
    }

    fn move_to(x: i32, y: i32) -> MessageResult {
        println!("Move to: ({:?}, {:?})", x, y);
        MessageResult::Success
    }

    fn change_color(r: i32, g: i32, b: i32) -> MessageResult {
        println!("Change color to: ({:?}, {:?}, {:?})", r, g, b);
        MessageResult::Success
    }

    fn write(text: &String) -> MessageResult {
        println!("Write: {}", text);
        MessageResult::Success
    }
}

fn safe_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

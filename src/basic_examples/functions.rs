pub fn test(){
    func_with_no_arg();
    println!("Function - with arg, res = {:?}", func_with_arg_and_return_type(2, 2));
}

fn func_with_no_arg() {
    println!("Function - with no arg");
}

fn func_with_arg_and_return_type(a: i32, b: i32) -> i32 {
    if a % 2 == 0 {
        return 2 * (a + b); // return can be used too
    }

    a + b // expression (without ;) is used for return
}
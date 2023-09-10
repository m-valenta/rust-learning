pub fn test() {
    let s1 = String::from("hello");
    let mut s2 = String::from("hello");

    let s1_ref = &s1;
    let s2_mut_ref = &mut s2;

    try_refs(s1_ref, s2_mut_ref);

    println!("Result: {:?}", s2);
}

fn try_refs(s1: &String, s2: & mut String) {
    s2.push_str(s1);
}



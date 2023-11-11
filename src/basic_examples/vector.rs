pub fn test() {
    let mut v = vec![1, 2, 3, 4, 5];

    println!("Initial v = {:?}", v);

    if let Some(first) = v.get_mut(0) {
        *first *= 10;
    }

    println!("After get_mut v = {:?}", v);

    let second = &mut v[1];
    *second = *second * 10;

    println!("After &mut v[] v = {:?}", v);

    for value in &mut v {
        *value *= 2;
    }

    println!("After mod iterator v = {:?}", v);

    let mut sum = 0;
    for value in &v {
        sum += *value;
        
        // Same as
        // sum += value;
    }

    println!("Sum of v = {}", sum);

    let mut slice: &mut [isize] = &mut v;
    slice[0] = 0;
    
    slice = &mut v[2..4]; // &mut v is slice too 
    slice[1] = 0;

    println!("After slicing v = {:?}", v);
}

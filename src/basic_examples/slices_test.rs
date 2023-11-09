pub fn test() {
    const TEST_TEXT: &str = "Hello world  ";

    println!("Slices test1 - result: {:?}", get_word_on_index(TEST_TEXT, 0));
    println!("Slices test1 - result: {:?}", get_word_on_index(TEST_TEXT, 1));
    println!("Slices test1 - result: {:?}", get_word_on_index(TEST_TEXT, 2));

    let mut test_numbers = [1;3];
    println!("Slices test2 - original: {:?}", test_numbers);

    let test_slice = &mut test_numbers[1..];
    
    // We can modify collection slice
    test_slice[0] = 2;
    test_slice[1] = 2;


    println!("Slices test2 - result: {:?}", test_numbers);
}

fn get_word_on_index(input: &str, w_index: isize) -> &str {

    let mut current_word = 0;
    let mut start_index = 0;

    for (chr_index, chr) in input.chars().enumerate() {
        if chr != ' ' {
            continue;
        }

        if w_index == current_word {
            return &input[start_index..chr_index];
        }

        current_word += 1;
        start_index = chr_index + 1;
    }

    return &input[start_index..];
}
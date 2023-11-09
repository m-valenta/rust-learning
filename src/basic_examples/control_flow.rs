pub fn test(){
    println!("If - result: {:?}", if_constructs(12));
    println!("While - result: {:?}", while_construct(5));
    println!("For - result: {:?}", for_construct());
}

fn if_constructs(num: i32) -> String{
    
    // if is expression, can be assigned 
    let num_calc = if num < 5 {
        num // return num as the result of the block
    } else {
        5
    };


    if num_calc < 2 {
        "Low".to_string()
    } else if num < 5 {
        "Medium".to_string()
    } else{
        "High".to_string()
    }
}

fn while_construct(max_iter: i32)->String {
    let mut iter_cnt = 0;
    let mut result: String = String::from("");

    'outer: while iter_cnt < max_iter {
        let mut inner_cnt= 0;
        while inner_cnt < 10 {
            iter_cnt += 1;

            if inner_cnt == iter_cnt{
                break 'outer;
            }

            result.push('i');
            inner_cnt += 1;
        }
    }

    result.to_string()
}

fn for_construct() -> String {
    let mut result = String::from("");
    let test = ["a", "b", "c"];

    // Take range <0,4)
    // Using reversing function
    for i in (0..4).rev() {
        result.push_str(&i.to_string());
    }

    result.push(';');

    for item in test {
        result.push_str(&item);
    }

    result
}
pub fn test() {
    
    // Types + length
    let a: [[i32; 2]; 2] = [
        [0, 1],
        [2, 3]
    ];

    // Using initialization
    let b = [[1;2];2];

    println!("Arrays contents a: {:?}, b: {:?}", a, b);
    println!("Arrays indexing: a[1][1] = {:?}", a[1][1]);


    let mut c = [[1;2];2];
    c[0][1] = 2;
    c[1][1] = 2;


    const WIDTH: usize = 2;
    const HEIGHT: usize = 1;

    let mut d: [[i32; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    d[0][0] = 1;
    d[0][1] = 2;


    println!("Array contents c: {:?}, d: {:?}", c, d);
    println!("Array Lengths: d.len(): {:?}, d[0].len(): {:?}", d.len(), d[0].len());
}
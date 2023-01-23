fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // let y = x + 10;
    // println!("The value of y is: {y}");

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("The value of THREE_HOURS_IN_SECOND is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing example
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Shadowing another example
    let spaces = "   ";
    let spaces = spaces.len();
    
    println!("The value of spaces is {spaces}");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("{sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("{t}, {f}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{c}, {z}, {heart_eyed_cat}");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {x}, y is: {y}, z is: {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{five_hundred}, {six_point_four}, {one}");
}

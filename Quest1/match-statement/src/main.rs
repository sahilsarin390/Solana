fn main() {

    // Match statement

    let x = 3;

    match x {
        1 => println!("Value of x is 1"),
        2 => println!("Value of x is 2"),
        _ => println!("Value of x is invalid"),
    }

    // Exhaustive matches:

    let a = true;
    let b = false;

    if a && b {
        println!("a and b are true");
    } else if !a && !b {
        println!("a and b are false");
    } else if a && !b {
        println!("a is true, b is false");
    } else if !a && b {
        println!("a is false, b is true");
    } // This method mith cause bugs

    // Best method:

    match (a, b) {
        (true, true) => println!("a and b are true"),
        (false, false) => println!("a and b are false"),
        (true, false) => println!("a is true, b is false"),
        (false, true) => println!("a is false, b is true"),
    }
}   

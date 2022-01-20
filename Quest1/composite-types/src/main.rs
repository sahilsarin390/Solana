fn main() {

    // Tuples
    let mut a = (5, "Hey", true);
    println!("The first two values are: {} and {}",a.0,a.1);

    //Another way
    a = (55,"Hey yo", false);
    let val_one = a.0;
    let val_two = a.1;
    println!("The first value is: {} and The second value is: {}", val_one, val_two);

    // Another one

    // let(val_one, val_two, _) = a  //underscore bcz three vals req

    // Arrays
    // A set of same type of values

    let b = [1, 2, 3, 4, 5];
    println!("The first value is: {}", b[0]);

    // if value repeats:

    let b = [0; 10];
    println!("The array is: {:?}", b); //:? is debugging

    let b: [i32;10] = [69; 10];  // i32 is datatype, 10 is no. of elements
    println!("The array is: {:?}", b); //:? is debugging
}

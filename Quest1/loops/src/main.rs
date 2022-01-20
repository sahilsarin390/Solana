fn main() {

    //loops

    let mut x =1;

    loop {

        x = x * 2;
        if x > 10000 { 
            break;
        }

        println!("The value of x now is {}",x);
    }

    // While loops
    let mut y = 1;
    while y < 10000 {

        y = y * 2;
        println!("The value of y now is {}",y);

    }

    // for loops

    for a in 0..10 {
        println!("The value of a is {}",a);
    }
    
    // More explicit

    for b in 0..=10 { // inclusive

        println!("The value of b is {}",b);

    }

    // Another
    
    let c = [1, 2, 3];

    for val in c {
        println!("The value of is {}",val);
    }

    println!("Outside the loops");
}

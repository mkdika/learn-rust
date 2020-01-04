fn main() {

    // immutable variable
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6 // re-assign this will cause compile error

    // mutable variable
    let mut y = 1;
    println!("The value of y is: {}", y);
    y = 3;
    println!("The value of y is: {}", y);

    // const
    const MAX_POINTS: u32 = 100_000;
    println!("The value const MAX_POINT is: {}", MAX_POINTS);

    // shadowing immutable variable, using let syntax
    let n = 5;
    let n = n + 1;
    let n = n * 2;
    println!("The value of n is: {}", n);
}

fn main() {
    another_function(5);
    println!("a + b = {}", sum(5,3));
}

// non return function
fn another_function(x: i32) {
    println!("the value of x is: {}", x);
}

// return function
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
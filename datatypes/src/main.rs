fn main() {
    // integer
    let x = 10; // i32
    println!("x is {}", x);
    let y: u64 = 10_000;
    println!("y is {}", y);

    // floating point
    let a = 2.75; // f64
    println!("a is {}", a);
    let b: f32 = 3.15; // f32
    println!("b is {}", b);

    // numeric operations
    // addition
    let sum = 5 + 10;
    println!("sum is {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference is {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("multiplication is {}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("quotient is {}", quotient);
    // remainder
    let remainder = 43 % 5;
    println!("remainder is {}", remainder);

    // boolean
    let t = true;
    println!("t is {}", t);
    let f: bool = false;
    println!("f is {}", f);

    // chars
    let c = 'z';
    println!("c is {}", c);
    let thumb = '\u{01f44d}';
    println!("thumb is {}", thumb);

    // string
    let name = "Maikel Chandika";
    println!("name is {}", name);

    // tuple
    let tup = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("tup x is {}, y is {}, z is {}", x,y,z);
}

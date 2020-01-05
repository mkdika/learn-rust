fn main() {
    // loop
    let mut i = 1;
    loop {
        println!("loop:{}", i);
        i += 1;
        if i >= 6 {
            break;
        }
    }

    // returing value from loop (expression)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("expression loop result is {}", result);

    // while
    let mut number = 3;
    while number >= 0 {
        println!("while {}!", number);
        number -= 1;
    }

    // looping through a collection
    let a = [10,20,30,40,50];
    for el in a.iter() {
        println!("element value is {}", el);
    }

    // looping from ranges
    for n in (0..10).rev() {
        println!("{}!", n);
    }
}

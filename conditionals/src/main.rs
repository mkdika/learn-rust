fn main() {
    // if..else as statement
    let number = 3;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    }

    // if..else as expression
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("number now is {}", number);
}

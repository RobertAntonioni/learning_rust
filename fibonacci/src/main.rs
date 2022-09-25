use std::io;
fn main() {
    loop {
    println!("\nWelcome to Fibonacci Printer");
    println!("Which position in the fibonacci sequence are you interested in?");

    let mut position = String::new();
    io::stdin().read_line(&mut position)
        .expect("Failed to read line");
    let position = position.trim();

    let position: u64 = match position.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input an integer!");
            continue;
        }
    };

    if position == 1 || position == 2 {
        println!("The value in position {} is equal to {}", position, 1);
        continue;
    }

    let mut fib: [u64; 2] = [1, 1];             // Create an array of two 64-bit unsigned integers and start them at [1, 1]
    let mut index = 3;                          // Create a variable named index initiated to 3
    while index <= position {                   // While index is less than or equal to position run the code in block
        let tmp = fib[0] + fib[1];              // Create a variable named tmp with the value of fib[0] + fib[1]
        fib[0] = fib[1];                        // Assign the value of fib[1] to fib[0]
        fib[1] = tmp;                           // Assign the value of tmp to fib[1]
        index += 1;                             // Index is now equal to Index + 1
    }
    println!("The value in position {} is equal to {}", position, fib[1]);
    }
}
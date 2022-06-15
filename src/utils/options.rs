use std::io::{self, Write};

pub fn option(mut input: String) -> String {
    println!("Hello, Please enter the number: ");
    println!("1. Normal IFPS ");
    println!("2. Normal IFPS + Cluster ");
    print!("Enter a number >> ");
    io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Error reading from STDIN");

    return input;
}

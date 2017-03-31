use std::io;

fn main() {
    println!("This is Josh's Project Euler in Rust program");
    println!("Enter the number of the problem you wold like to run:");

    let mut eulerproblem = String::new();

    io::stdin().read_line(&mut eulerproblem)
        .expect("Failed to read line");

    let guess: u32 = eulerproblem.trim().parse()
        .expect("Please type a number!");


    println!("Running: {}", eulerproblem);
    println!("...");	
}

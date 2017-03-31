use std::io;
use std::cmp::Ordering;

fn main() {
    println!("This is Josh's Project Euler in Rust program");
    const CHOICE: u32 = 0;


    loop {		   
        println!("Enter 0 to exit or the number of the problem you wold like to run:");

        let mut eulerproblem = String::new();

        io::stdin().read_line(&mut eulerproblem)
            .expect("Failed to read line");

        let eulerproblem: u32 = match eulerproblem.trim().parse() {
            Ok(num) => num,
	    Err(_) => continue,
        };
        match eulerproblem.cmp(&CHOICE) {
            Ordering::Greater => {
                println!("Running: {}", eulerproblem);
                println!("...");	
            },
            Ordering::Less   => println!("Too small"),
            Ordering::Equal    => break,
            
        }

    }
}

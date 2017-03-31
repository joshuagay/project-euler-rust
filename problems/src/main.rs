use std::io;

fn main() {
    println!("This is Josh's Project Euler in Rust program");

    loop {		   
        println!("Enter the number of the problem you wold like to run:");

        let mut eulerproblem = String::new();

        io::stdin().read_line(&mut eulerproblem)
            .expect("Failed to read line");

        let eulerproblem: u32 = match eulerproblem.trim().parse() {
            Ok(num) => num,
	    Err(_) => continue,
        };
    }

    println!("Running: {}", eulerproblem);
    println!("...");	
}

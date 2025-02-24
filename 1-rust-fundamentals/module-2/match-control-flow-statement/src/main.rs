use std::io;

fn main() {
    
    println!("Please, enter a greeting:");
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    // use match expression to pattern match agains variable "name"
    match name.trim() {
        "Good Bye" => println!("Sorry to see you go"),
        "Hello" => println!("Hi, nice to meet you"),
        _ => println!("I can't find a greeting, good bye")
    }
}

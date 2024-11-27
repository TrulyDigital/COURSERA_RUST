fn main() {

    //
    let mut i: i32 = 0;
    while i < 5 {
        println!("i is {}", i);
        i += 1;
    }

    //
    let mut input: String = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Please, enter a word (type 'stop' to exit): ");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("You entered: {}", input);
    }
    println!("Exiting the loop.");
}

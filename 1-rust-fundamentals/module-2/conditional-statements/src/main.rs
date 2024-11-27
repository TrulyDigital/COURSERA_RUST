fn main() {
    let maybe_number: Option<Option<()>> = Some(None);
    let maybe_number: Option<i32> = Some(42);
    if let Some(42) = maybe_number {
        println!("The number is: {:?}", 42);
    } 
    else if let Some(number) = maybe_number {
        println!("The number is: {:?}", number);
    }
    else {
        println!("No number found");
    }
}

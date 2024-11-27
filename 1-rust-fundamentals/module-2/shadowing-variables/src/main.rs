fn main() {
    
    let mut height: i32 = 100;
    height = height - 5;
    let result: &str = if height > 180 {
        "tall"
    } 
    else if height > 160 {
        "average"
    } 
    else {
        "short"
    };
    println!("{}", result);

    let healt: &str = if height < 180 {"Good"} else {"Unknown"};
    println!("{}", healt);

    // shadowing to a different type
    let height: bool = if height > 180 {true} else {false};
    println!("{}", height);
}

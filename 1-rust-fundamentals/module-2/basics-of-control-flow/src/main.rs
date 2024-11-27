fn main() {
    let proceed: bool = true;
    if proceed {
        println!("Proceeding!");
    } else {
        println!("Not preceeding!");
    }

    let height: i32 = 190;
    if height < 180 {
        println!("You are tall!");
    } else if height < 160 {
        println!("You are short!");
    } else {
        println!("You are average height!");
    }

    let age: i32 = 30;
    if age < 10 {
        println!("You are a child!");
    } else if age < 21 {
        println!("You are a teenager!");
    } else {
        println!("You are an adult!");
    }
}

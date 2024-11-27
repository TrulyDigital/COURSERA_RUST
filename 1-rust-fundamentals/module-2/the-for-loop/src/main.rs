use std::vec;

fn main() {

    // not inclusive
    for i in 1..10 {
        println!("i = {}", i);
    }

    println!("-------------------");

    // inclusive
    for x in 1..=10 {
        println!("x = {}", x);
    }

    println!("-------------------");

    // reverse + inclusive
    for y in (1..=10).rev() {
        println!("y = {}", y);
    }

    println!("-------------------");

    // vector
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    for n in numbers {
        println!("numbers = {}", n);
    }
}

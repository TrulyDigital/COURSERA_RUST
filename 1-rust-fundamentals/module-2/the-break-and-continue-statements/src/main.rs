fn main() {
    for i in 1..=10 {

        if i % 2 == 0 {
            // skip even numbers
            continue;
        }

        println!("i= {}", i);
        if i == 7 {
            // stop the loop when i is 7
            break;
        }
    }
}

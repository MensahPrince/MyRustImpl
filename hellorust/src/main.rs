use std::io;


fn hello() {
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read name");
    if name.trim() == "" {
        println!("Please enter your name:");
        return;
    }
    println!("Hello, {}!", name.trim());
}
fn main() {
    hello();
}


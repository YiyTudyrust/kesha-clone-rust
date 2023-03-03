use std::io;
use std::io::stdin;

fn main() {
    let name_of_app:&str = "Kesha-clone-rust";
    println!("Hay, welcome to {name_of_app}.This app can repeat your answer!");
    println!("Please input anythings.");
    let mut anything = String::new();
    io::stdin().read_line(&mut anything).expect("Failed to read line!");
    println!("{name_of_app} say: {anything}");
}

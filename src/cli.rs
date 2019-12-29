use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Amit";
    let status = "100%";

    println!("Args: {:?}", args); // First parameter returned is the path of executable rust file

    println!("Command: {:?}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status: {}", status);
    } else {
        println!("That is not a valid command!");
    }
}

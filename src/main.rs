// importing module print.rs
mod print; // print is file name
mod strings;
mod types;
mod variables;

fn main() {
    println!(":::::String formatting:::::");
    print::greet(); // basically calls greet() inside print.rs

    println!(":::::Variables in Rust:::::");
    variables::run();

    println!(":::::Types:::::");
    types::run();

    println!(":::::Strings and its Methods:::::");
    strings::run();
}

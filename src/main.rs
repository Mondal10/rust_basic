// importing module print.rs
mod arrays; // arrays is file name
mod print; // print is file name
mod strings;
mod tuples;
mod types;
mod variables;
mod vectors;

fn main() {
    println!(":::::String formatting:::::");
    print::greet(); // basically calls greet() inside print.rs

    println!(":::::Variables in Rust:::::");
    variables::run();

    println!(":::::Types:::::");
    types::run();

    println!(":::::Strings and its Methods:::::");
    strings::run();

    println!(":::::Tuples:::::");
    tuples::run();

    println!(":::::Arrays:::::");
    arrays::run();

    println!(":::::Vectors:::::");
    vectors::run();
}

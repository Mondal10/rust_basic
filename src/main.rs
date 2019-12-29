// importing modules
mod arrays; // arrays is file name
mod cli;
mod conditionals;
mod enums;
mod functions;
mod loops;
mod pointers;
mod print;
mod strings;
mod structs;
mod tuples;
mod types;
mod variables;
mod vectors;

fn main() {
    others();
    println!(":::::CLI:::::");
    cli::run();
}

fn others() {
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

    println!(":::::Conditionals:::::");
    conditionals::run();

    println!(":::::Loops:::::");
    loops::run();

    println!(":::::Functions:::::");
    functions::run();

    println!(":::::Pointers:::::");
    pointers::run();

    println!(":::::Structs:::::");
    structs::run();

    println!(":::::Enums:::::");
    enums::run();
}

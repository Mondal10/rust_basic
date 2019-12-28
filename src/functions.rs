// Reusable blocks of codes

pub fn run() {
    greetings("Hello", "Amit Mondal");

    // Bind functio values to variable
    let get_sum = add(5, 1);
    println!("Sum: {}", get_sum);

    // Closures
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; // Due to closure n3 can also be accessed
    println!("Closure Sum: {}", add_nums(3, 2));
}

fn greetings(greet: &str, name: &str) {
    println!("{} {} nice to meet you!", greet, name);
}

// `->` is to denote the return type of the function
fn add(n1: i32, n2: i32) -> i32 {
    // Do not use semicolon(;) to indicate what to return
    n1 + n2
}

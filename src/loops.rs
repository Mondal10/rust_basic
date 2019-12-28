pub fn run() {
    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("Count: {}", count);

        if count == 20 {
            break;
        }
    }

    let mut count_while = 0;

    // While loop (FizzBuzz)
    while count_while <= 100 {
        if count_while % 15 == 0 {
            println!("FizzBuzz");
        } else if count_while % 3 == 0 {
            println!("Fizz");
        } else if count_while % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count_while);
        }

        count_while += 1;
    }

    // For range loop
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}

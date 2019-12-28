// Conditionals are used to check the conditions of something and act accordingly

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = true;
    let knows_age_of_person: bool = true;

    // If/Else If/Else
    if (age >= 21 && check_id) || knows_age_of_person {
        println!("You can enter the pub!");
    } else if age < 21 && check_id {
        println!("Sorry you cannot enter!");
    } else {
        println!("Show your ID!");
    }

    // Short-hand If *No ternary operator*
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of Age: {}", is_of_age);
}

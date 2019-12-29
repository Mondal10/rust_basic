// Enums are types which have a few definite values.

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Perform action
    match m {
        // match is somewhat similar to switch case
        Movement::Up => println!("Avatar Moving UP"),
        Movement::Down => println!("Avatar Moving Down"),
        Movement::Left => println!("Avatar Moving Left"),
        Movement::Right => println!("Avatar Moving Right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Up;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Left;
    let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}

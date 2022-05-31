// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed

use std::io;

// * Use an enum to store the possible power states
enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

// * Use a match expression to convert the user input into the power state enum
impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        // * The program should be case-insensitive (the user should be able to type
        //   Reboot, reboot, REBOOT, etc.)
        let state = state.trim().to_lowercase();
        // String -> &str
        match state.as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        }
    }
}

// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
fn print_power_action(state: PowerState) {
    use PowerState::*;
    match state {
        Off => println!("turning off"),
        Sleep => println!("sleeping"),
        Reboot => println!("rebooting"),
        Shutdown => println!("shutting down"),
        Hibernate => println!("hibernating"),
    }
}

fn main() {
    let mut buffer = String::new();
    println!("Enter new power state:");
    let user_input = io::stdin().read_line(&mut buffer);
    if user_input.is_ok() {
        // * Use a match expression to convert the user input into the power state enum
        match PowerState::new(&buffer) {
            Some(state) => print_power_action(state),
            None => println!("invalid power state"),
        }
    } else {
        println!("error reading input");
    }
}

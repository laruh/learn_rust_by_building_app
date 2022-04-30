// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100

// * Use a function to print the messages
fn print_answer(rsl: bool) {
    // * Use a match expression to determine which message
    //to print
    match rsl {
        true => println!("its big"),
        false => println!("its small")
    }
}

fn main() {
    let var = 150;
    let rsl = var > 100;
    print_answer(rsl);
}
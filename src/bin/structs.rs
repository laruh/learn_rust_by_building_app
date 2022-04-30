// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces

// * Use an enum to create different flavors of drinks
enum Flavors {
    Sweet,
    Sparkling,
    Fruity
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavors,
    ounces: f64
}

// * Use a function to print out the drink flavor and ounces
fn print_flavor_ounce (drink: Drink) {
    // * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavors::Sweet => println!("sweet"),
        Flavors::Sparkling => println!("sparkling"),
        Flavors::Fruity => println!("fruity")
    }
    println!("{:?}", drink.ounces);
}

fn main() {
    let coke = Drink {
        flavor: Flavors::Sweet,
        ounces: 8.0
    };
    print_flavor_ounce(coke);
}
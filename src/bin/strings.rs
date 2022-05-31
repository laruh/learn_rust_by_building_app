// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under

// * Use a struct for a persons age, name, and favorite color
struct Person {
    age: i32,
    // * The color and name should be stored as a String
    name: String,
    favorite_color: String,
}

impl Person {
    fn new(age: i32, name: String, favorite_color: String) -> Self {
        Self {
            age,
            name,
            favorite_color,
        }
    }
}

fn print_info(data: &str) {
    println!("{}", data);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person::new(30, "Alex".to_owned(), "Red".to_owned()),
        Person::new(16, "Maria".to_owned(), "Blue".to_owned()),
        Person::new(25, "Sam".to_owned(), "Green".to_owned()),
    ];

    // * Iterate through the vector using a for..in loop
    for person in people {
        // * Use an if expression to determine which person's info should be printed
        // * The name and colors should be printed using a function
        if person.age > 20 {
            print_info(&person.name);
            print_info(&person.favorite_color);
        }
    }
}

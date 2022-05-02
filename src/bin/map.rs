// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User

#[derive(Debug)]
struct User {
    id: i32,
    name: String
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    match name.to_lowercase().as_str() {
        "sam" => Some(1),
        "alex" => Some(5),
        "nikol" => Some(10),
        _ => None
    }
}

fn main() {
    let user_name = "sam";
    let user = find_user(user_name)
        .map(|user_id| User {id: user_id, name: user_name.to_owned()});

    // * Print out the User struct if found, or a "not found" message if not
    match user {
        Some(user) => println!("We have user {:?}", user),
        None => println!("User not found")
    }
}
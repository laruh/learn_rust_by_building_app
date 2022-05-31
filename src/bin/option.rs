// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let sam = Student {
        name: "Sam".to_owned(),
        locker: Some(32),
    };
    println!("student: {:?}", sam.name);
    match sam.locker {
        Some(num) => println!("student's locker {:?}", num),
        None => println!("no locker assigned"),
    }
}

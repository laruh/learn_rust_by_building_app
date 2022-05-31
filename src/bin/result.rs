// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21

// * Use a struct to store at least the age of a customer
struct Customer {
    age: i32,
}
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
fn try_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age < 21 {
        // * The Err variant should detail the reason why they cannot make a purchase
        Err("Customer must be at least 21 age".to_owned())
    } else {
        Ok(())
    }
}

fn main() {
    let sam = Customer { age: 22 };
    let purchase = try_purchase(&sam);
    println!("{:?}", purchase)
}

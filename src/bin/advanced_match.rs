// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price

// * Use an enum for the tickets with data associated with each variant
enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    // * Create one of each ticket and place into a vector
    let tickets = vec![
        Ticket::Backstage(100.0, "Sam".to_owned()),
        Ticket::Vip(90.0, "Alex".to_owned()),
        Ticket::Standard(50.5),
    ];
    for ticket in tickets {
        // * Use a match expression while iterating the vector to print the ticket info
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Backstage ticket holder {:?}, price {:?}", holder, price)
            }
            Ticket::Vip(price, holder) => {
                println!("Vip ticket holder {:?}, price {:?}", holder, price)
            }
            Ticket::Standard(price) => println!("Standard ticket price {:?}", price),
        }
    }
}

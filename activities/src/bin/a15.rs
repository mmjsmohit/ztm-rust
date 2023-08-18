// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Backstage and Vip tickets include the ticket holder's name
//
// Notes:

// * Use an enum for the tickets with data associated with each variant
enum Tickets {
    // * Tickets can be Backstage, Vip, and Standard
    // * All tickets include the price
    Backstage(String),
    Vip(String),
    Standard,
}

fn main() {
    // * Create one of each ticket and place into a vector
    let tickets = vec![
        Tickets::Backstage("Mohit".to_owned()),
        Tickets::Vip("Garvit".to_owned()),
        Tickets::Standard,
    ];

    // * Use a match expression while iterating the vector to print the ticket info
    for ticket in tickets {
        match ticket {
            Tickets::Backstage(x) => println!("{:?} is having Backstage access!", x),
            Tickets::Vip(x) => println!("{:?} is having VIP access!", x),
            Tickets::Standard => println!("This is a standard access ticket!"),
        }
    }
}

// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

#[derive(Debug)]
enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64)
}


fn main() {
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
let tickets = vec![
    Ticket::Backstage(50.0, "Reuben".to_owned()),
    Ticket::Vip(20.0, "Isha".to_owned()),
    Ticket::Standard(50.0),
];

for ticket in tickets{
    println!("{:?}", ticket)
}
}

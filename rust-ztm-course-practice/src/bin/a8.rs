// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    vanilla,
    orange,
    strawberry
}

struct Drink {
    flavor: Flavor,
    ounces: f64
}

fn print_drink_details (drink: Drink){

    match drink.flavor {
        Flavor::orange => println!("Orange!"),
        Flavor::vanilla => println!("Vanilla!"),
        Flavor::strawberry => println!("Strawberry!")
    }
}

fn main() {
    let mut drink = Drink{
        flavor: Flavor::strawberry,
        ounces: 7.22
    };

    print_drink_details(drink);
}

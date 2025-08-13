// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function


// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    age: i32,
    name: String,
    fav_color: String
}


fn print(data: &str){
    println!("{:?}", data)
}


fn main() {
// * Create and store at least 3 people in a vector
let people = vec![
    Person{
        age: 3,
        name: String::from("Reuben"),
        fav_color: "Blue".to_owned()
    },
    Person{
        age: 24,
        name: String::from("Reuben-2"),
        fav_color: "Green".to_owned()
    },
    Person{
        age: 26,
        name: String::from("Reuben-3"),
        fav_color: "Red".to_owned()
    },
];

// * Iterate through the vector using a for..in loop
for person in people{
    // * Use an if expression to determine which person's info should be printed
    // * Print out the name and favorite colors of people aged 10 and under
    // * The name and colors should be printed using a function

    if person.age <= 10 {println!("Name:{:?} \n Fave Color:{:?}",person.name, person.fav_color)}

    print(&person.name);

}

}
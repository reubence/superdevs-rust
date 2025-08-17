// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

// * Use a struct to store at least the age of a customer
#[derive(Debug)]
struct Adult{
    name: String,
    age: u8
}

impl Adult{
    fn new(name:&str, age:u8) -> Result<Self, &str>{
        if age >= 21 {
            Ok(Self{
                age,
                name: name.to_string()
            })
        }else {
            Err("The person is not ellgible")
        }
    }
}

// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
fn main() {

    let child = Adult::new("Reuben", 12);
    let child = Adult::new("Isha", 22);

    match child {
        Ok(child) => println!("{:?}", child),
        Err(err) => println!("{:?}", err)
    }

}


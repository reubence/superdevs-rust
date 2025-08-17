// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
#[derive(Debug)]
struct Locker {
    name: String,
    assignment: Option<i32>
}

fn main() {
    let student = Locker{
        name: "Reuben".to_owned(),
        assignment:Some(32)
    };

    println!("{:?}", student)
}

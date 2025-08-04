
struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

fn full_name(&self) -> String {
        impl User {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn static_age()->String{
        String::from("Hello")
    }
}

enum Shape{
    Rectangle(f64, f64),
    Circle(f64)
}

fn print_area(shape: Shape){
}

fn main() {
    // println!("{}", is_even(2));
    println!("{}", fib(4));

    let name = String::from("reuben");
    let len = get_str_len(&name);

    println!("{}", len);

    let user = User {
        first_name: String::from("Reuben"),
        last_name: String::from("Rapose"),
        age: 30
    };

    println!("{}", user.first_name);
    println!("{}", user.full_name());
    println!("{}", User::static_age());

    let rect = Shape::Rectangle(1.0, 2.0);
    print_area(shape: rect);
    let circle = Shape::Circle(3.2);
    print_area(shape: circle);
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn fib(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }
    if num == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=num {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}

fn get_str_len(s: &str) -> usize {
    s.chars().count()
}
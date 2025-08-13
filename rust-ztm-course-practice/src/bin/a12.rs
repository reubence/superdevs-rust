// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// * Use an enum for the box color
enum Color {
    white,
    black,
    green
}

impl Color {
    fn print(&self) {
        match self{
            Color::black => println!("Color is black"),
            Color::white => println!("Color is white"),
            Color::green => println!("Color: green")
        }
    }
}
// * Use a struct to encapsulate the box characteristics
// * Must include dimensions, weight, and color
struct ShippingBox {
    dimensions: f32,
    weight: f32,
    color: Color
}

// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
impl ShippingBox {
    fn new() -> Self {
        Self {
            dimensions: 22.4,
            weight: 32.0,
            color: Color::green
        }
    }
    
    fn print_details(&self) {
        println!("Dimensions: {:?} \n Weight: {:?} \n",self.dimensions, self.weight);
        self.color.print()
    }
}
fn main() {
    let new_box = ShippingBox::new();
    new_box.print_details();
}

// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item 
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
struct GroceryItem {
    qty: i32,
    id: i32
}

// * Create a function to display the quantity, with the struct as a parameter
fn display_qty (item: &GroceryItem){
    println!("{:?}",item.qty)
}
// * Create a function to display the id number, with the struct as a parameter
fn display_id (item: &GroceryItem){
    println!("{:?}",item.id)
}
fn main() {
// * Print out the quantity and id number of a grocery item
let grocery_item = GroceryItem{
    qty: 10,
    id: 212
};

display_id(&grocery_item);
display_qty(&grocery_item);
}

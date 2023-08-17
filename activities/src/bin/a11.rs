// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter


// * Use a struct for the grocery item
struct GroceryItem {
    // * Use two i32 fields for the quantity and id number
    quantity: i32,
    id: i32
}

fn display_quantity(item: &GroceryItem) {
    println!("{:?} is the quantity.", item.quantity);
}

fn display_id(item: &GroceryItem) {
    println!("{:?} is the ID.", item.id);
}
fn main() {
    let item = GroceryItem {quantity: 30, id: 10};
    display_quantity(&item);
    display_id(&item);
}

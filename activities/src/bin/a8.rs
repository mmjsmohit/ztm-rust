// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:


// * Use an enum to create different flavors of drinks
enum Flavor {
    Coke,
    Pepi,
    Fanta
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor:Flavor,
    ounce: f64
}

// * Use a function to print out the drink flavor and ounces
fn printInfo(myDrink: Drink) {
    // * Use a match expression to print the drink flavor
    match myDrink.flavor {
        Flavor::Coke => println!("Coke"),
        Flavor::Pepi => println!("Pepsi"),
        Flavor::Fanta => println!("Fanta")
    }
    println!("Capacity is {:?}",myDrink.ounce);
}


fn main() {
    let myDrink = Drink{
        flavor: Flavor::Coke,
        ounce: 34.0
    };
printInfo(myDrink);
}

// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:

// * Use a struct for a persons age, name, and favorite color
struct Person {
    age: i32,
    // * The color and name should be stored as a String
    name: String, //String in struct should be an owned String.
    fav_color: String,
}

// * The name and colors should be printed using a function
fn print_name(person: &Person) { //The function can also accept String here. It should accept &str which is a borrowed data.
    println!("{:?} is the name.", person.name);
}
fn print_color(person: &Person) {
    println!("{:?} is the favourite color.", person.fav_color);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            age: 20,
            name: "Mohit".to_owned(), //String::from("Mohit") also works!
            fav_color: "Orange".to_owned(),
        },
        Person {
            age: 5,
            name: "Shaunak".to_owned(),
            fav_color: "Black".to_owned(),
        },
        Person {
            age: 22,
            name: "Yugam".to_owned(),
            fav_color: "Green".to_owned(),
        },
    ];
    // * Iterate through the vector using a for..in loop
    for person in people {
        // * Use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            print_name(&person);
            print_color(&person);
        }
    }
}

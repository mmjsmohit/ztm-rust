// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:

// * Use a struct containing the student's name and locker assignment
struct Locker {
    name: String,
    // * The locker assignment should use an Option<i32>
    number: Option<i32>,
}

fn main() {
    let first_locker = Locker {
        name: "Mohit".to_owned(),
        number: None,
    };
    let second_locker = Locker {
        name: "Shaunak".to_owned(),
        number: Some(100),
    };
    match first_locker.number {
        Some(num) => println!(
            "{:?} owns this locker with locker number {:?}",
            first_locker.name, num
        ),
        None => println!(
            "{:?} owns this locker with no locker number",
            first_locker.name
        ),
    }
    match second_locker.number {
        Some(num) => println!(
            "{:?} owns this locker with locker number {:?}",
            second_locker.name, num
        ),
        None => println!(
            "{:?} owns this locker with no locker number",
            second_locker.name
        ),
    }
}

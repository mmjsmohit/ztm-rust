// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum Color {
    White,
    Orange,
    Blue
}


fn main() {
    let go = Color::White;
    match go {
        Color::White => println!("White"),
        Color::Orange => println!("Orange"),
        Color::Blue => println!("Blue")
        
    }
}

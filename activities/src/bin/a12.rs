// Topic: Implementing functionality with the impl keyword
//
// Requirements:
//
// Notes:

// * Use an enum for the box color
enum Color {
    Blue,
    White,
    Black,
    Brown
}

impl Color {
    fn print(&self){
        match self {
            Color::Blue => println!("Blue"),
            Color::White => println!("White"),
            Color::Black => println!("Black"),
            Color::Brown => println!("Brown")
        }
    }
}
// * Use a struct to encapsulate the box characteristics
struct Dimensions {
    breadth: i32,
    width: i32,
    height: i32
}

// * Print the characteristics of a shipping box
impl Dimensions {
    fn print(&self) {
        println!("Breadth: {:?}, Width: {:?}, Height: {:?}", self.breadth, self.width, self.height);
    }
}
// * Must include dimensions, weight, and color
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color
}
impl ShippingBox {
    // * Implement functionality on the box struct to create a new box
    fn new(dimensions: Dimensions,
        weight: f64,
        color: Color) -> Self {
            Self{dimensions, weight, color}
        }
        // * Implement functionality on the box struct to print the characteristics
        fn print(&self) {
            self.color.print();
            self.dimensions.print();
        }
}
fn main() {
    let small_dim = Dimensions {
        breadth: 1,
    width: 2,
    height: 3
    };
    let small_box = ShippingBox::new(small_dim, 50.0, Color::Blue);
    small_box.print();
}

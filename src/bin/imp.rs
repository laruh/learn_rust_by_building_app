// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color

// * Use an enum for the box color
enum Color {
    Red,
    Green,
    Yellow,
}

impl Color {
    fn print_color(&self) {
        match self {
            Color::Red => println!("red"),
            Color::Green => println!("green"),
            Color::Yellow => println!("yellow"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

// * Use a struct to encapsulate the box characteristics
struct SippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl Dimensions {
    fn print_dim(&self) {
        println!("width {}", self.width);
        println!("height {}", self.height);
        println!("depth {}", self.depth);
    }
}

// * Implement functionality on the box struct to create a new box
impl SippingBox {
    fn new(dimensions: Dimensions, weight: f64, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    // * Implement functionality on the box struct to print the characteristics
    fn print(&self) {
        self.dimensions.print_dim();
        println!("weight {}", self.weight);
        self.color.print_color();
    }
}

fn main() {
    let dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };
    let new_box = SippingBox::new(dimensions, 2.1, Color::Green);
    new_box.print();
}

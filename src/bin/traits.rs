// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

use std::ops::Mul;

trait Perimeter {
    fn calculate(&self) -> f64;
}

fn print_perimeter(thing: impl Perimeter) {
    println!("perimeter = {:?}", thing.calculate());
}

struct Square {
    side: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Triangle {
    fn check_existence(&self) -> bool {
        self.a + self.b > self.c && self.c + self.b > self.a && self.a + self.c > self.b
    }
}

impl Perimeter for Square {
    fn calculate(&self) -> f64 {
        self.side.mul(4.)
    }
}

impl Perimeter for Triangle {
    fn calculate(&self) -> f64 {
        self.a + self.c + self.b
    }
}

fn main() {
    let square = Square { side: 5. };
    let triangle = Triangle {
        a: 5.,
        b: 3.,
        c: 4.,
    };
    print_perimeter(square);
    if triangle.check_existence() {
        print_perimeter(triangle);
    } else {
        println!("figure is not a triangle")
    }
}

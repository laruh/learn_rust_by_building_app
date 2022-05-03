// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None
    } else { Some(a / b) }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_clamp() {
        let rsl = clamp(5, 6, 7);
        let exp = 6;
        assert_eq!(rsl, exp, "should be 6")
    }

    #[test]
    fn check_div() {
        let rsl = div(6, 2);
        let exp = Some(3);
        assert_eq!(rsl, exp, "should be 3")
    }

    #[test]
    fn check_concat() {
        let result = concat("a", "b");
        let expected = String::from("ab");
        assert_eq!(result, expected, "should be placed immediately adjacent");
    }
}
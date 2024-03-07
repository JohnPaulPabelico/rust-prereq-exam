// Function: add
// Description: Adds two integers and returns the result.
// Parameters:
// - a: The first integer operand.
// - b: The second integer operand.
// Returns: The sum of the two operands.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Function: subtract
// Description: Subtracts one integer from another and returns the result.
// Parameters:
// - a: The integer from which to subtract.
// - b: The integer to subtract.
// Returns: The result of the subtraction.
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

// Function: multiply
// Description: Multiplies two integers and returns the result.
// Parameters:
// - a: The first integer operand.
// - b: The second integer operand.
// Returns: The product of the two operands.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// Function: divide
// Description: Divides one integer by another and returns the result as a float.
// Parameters:
// - a: The dividend.
// - b: The divisor.
// Returns: The result of the division as a float.
pub fn divide(a: i32, b: i32) -> f32 {
    (a / b) as f32
}

// Function: modulus
// Description: Computes the modulus of one integer with another and returns the result.
// Parameters:
// - a: The dividend.
// - b: The divisor.
// Returns: The modulus of the division operation.
pub fn modulus(a: i32, b: i32) -> i32 {
    a % b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(4, 4), 8);
        assert_eq!(add(3, 4), 7);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(4, 2), 2);
        assert_eq!(subtract(10, 2), 8);
        assert_eq!(subtract(9, 4), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(6, 6), 36);
        assert_eq!(multiply(4, 2), 8);
        assert_eq!(multiply(9, 8), 72);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 5), 2.0);
        assert_eq!(divide(20, 4), 5.0);
        assert_eq!(divide(81, 3), 27.0);
    }

    #[test]
    fn test_modulus() {
        assert_eq!(modulus(5, 2), 1);
        assert_eq!(modulus(11, 3), 2);
        assert_eq!(modulus(29, 5), 4);
    }
}

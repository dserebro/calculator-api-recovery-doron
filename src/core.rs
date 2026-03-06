//! Core calculator functions.
//!
//! Pure arithmetic functions mirroring Python's `core.py`.

use std::fmt;

/// Error returned when dividing by zero.
#[derive(Debug, Clone)]
pub struct DivideByZeroError;

impl fmt::Display for DivideByZeroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot divide by zero")
    }
}

impl std::error::Error for DivideByZeroError {}

/// Add two numbers.
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtract b from a.
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiply two numbers.
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Divide a by b.
///
/// Returns an error if b is zero.
pub fn divide(a: f64, b: f64) -> Result<f64, DivideByZeroError> {
    if b == 0.0 {
        return Err(DivideByZeroError);
    }
    Ok(a / b)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- add tests ---

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-2.0, -3.0), -5.0);
    }

    #[test]
    fn test_add_mixed_signs() {
        assert_eq!(add(-2.0, 3.0), 1.0);
    }

    #[test]
    fn test_add_with_zero() {
        assert_eq!(add(0.0, 5.0), 5.0);
        assert_eq!(add(5.0, 0.0), 5.0);
    }

    #[test]
    fn test_add_both_zero() {
        assert_eq!(add(0.0, 0.0), 0.0);
    }

    // --- subtract tests ---

    #[test]
    fn test_subtract_positive_numbers() {
        assert_eq!(subtract(5.0, 3.0), 2.0);
    }

    #[test]
    fn test_subtract_negative_numbers() {
        assert_eq!(subtract(-5.0, -3.0), -2.0);
    }

    #[test]
    fn test_subtract_mixed_signs() {
        assert_eq!(subtract(-2.0, 3.0), -5.0);
    }

    #[test]
    fn test_subtract_with_zero() {
        assert_eq!(subtract(5.0, 0.0), 5.0);
        assert_eq!(subtract(0.0, 5.0), -5.0);
    }

    // --- multiply tests ---

    #[test]
    fn test_multiply_positive_numbers() {
        assert_eq!(multiply(2.0, 3.0), 6.0);
    }

    #[test]
    fn test_multiply_negative_numbers() {
        assert_eq!(multiply(-2.0, -3.0), 6.0);
    }

    #[test]
    fn test_multiply_mixed_signs() {
        assert_eq!(multiply(-2.0, 3.0), -6.0);
    }

    #[test]
    fn test_multiply_with_zero() {
        assert_eq!(multiply(0.0, 5.0), 0.0);
        assert_eq!(multiply(5.0, 0.0), 0.0);
    }

    // --- divide tests ---

    #[test]
    fn test_divide_positive_numbers() {
        assert_eq!(divide(6.0, 3.0).unwrap(), 2.0);
    }

    #[test]
    fn test_divide_negative_numbers() {
        assert_eq!(divide(-6.0, -3.0).unwrap(), 2.0);
    }

    #[test]
    fn test_divide_mixed_signs() {
        assert_eq!(divide(-6.0, 3.0).unwrap(), -2.0);
    }

    #[test]
    fn test_divide_zero_numerator() {
        assert_eq!(divide(0.0, 5.0).unwrap(), 0.0);
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(5.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Cannot divide by zero");
    }

    #[test]
    fn test_divide_by_zero_with_zero_numerator() {
        let result = divide(0.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_divide_fractional_result() {
        assert_eq!(divide(1.0, 3.0).unwrap(), 1.0 / 3.0);
    }
}

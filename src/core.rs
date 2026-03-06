//! Core calculator functions.
//!
//! Pure arithmetic functions mirroring the behavior of the Python `core.py` module.

/// Error type for division by zero.
#[derive(Debug, Clone, PartialEq)]
pub struct DivideByZeroError;

impl std::fmt::Display for DivideByZeroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        Err(DivideByZeroError)
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1.0, -2.0), -3.0);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_add_mixed() {
        assert_eq!(add(-1.0, 3.0), 2.0);
    }

    #[test]
    fn test_subtract_positive() {
        assert_eq!(subtract(5.0, 3.0), 2.0);
    }

    #[test]
    fn test_subtract_negative() {
        assert_eq!(subtract(-1.0, -2.0), 1.0);
    }

    #[test]
    fn test_subtract_zero() {
        assert_eq!(subtract(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_subtract_result_negative() {
        assert_eq!(subtract(3.0, 5.0), -2.0);
    }

    #[test]
    fn test_multiply_positive() {
        assert_eq!(multiply(2.0, 3.0), 6.0);
    }

    #[test]
    fn test_multiply_negative() {
        assert_eq!(multiply(-2.0, 3.0), -6.0);
    }

    #[test]
    fn test_multiply_zero() {
        assert_eq!(multiply(5.0, 0.0), 0.0);
    }

    #[test]
    fn test_multiply_both_negative() {
        assert_eq!(multiply(-2.0, -3.0), 6.0);
    }

    #[test]
    fn test_divide_positive() {
        assert_eq!(divide(6.0, 3.0).unwrap(), 2.0);
    }

    #[test]
    fn test_divide_negative() {
        assert_eq!(divide(-6.0, 3.0).unwrap(), -2.0);
    }

    #[test]
    fn test_divide_fractional() {
        assert_eq!(divide(1.0, 3.0).unwrap(), 1.0 / 3.0);
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(5.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Cannot divide by zero");
    }

    #[test]
    fn test_divide_zero_by_nonzero() {
        assert_eq!(divide(0.0, 5.0).unwrap(), 0.0);
    }
}

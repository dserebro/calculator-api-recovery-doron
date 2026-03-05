/// Pure arithmetic functions equivalent to core.py.

pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(a: f64, b: f64) -> Result<f64, DivisionByZeroError> {
    if b == 0.0 {
        return Err(DivisionByZeroError);
    }
    Ok(a / b)
}

#[derive(Debug, Clone)]
pub struct DivisionByZeroError;

impl std::fmt::Display for DivisionByZeroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot divide by zero")
    }
}

impl std::error::Error for DivisionByZeroError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2.0, -3.0), -5.0);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_add_mixed() {
        assert_eq!(add(-2.0, 3.0), 1.0);
    }

    #[test]
    fn test_subtract_positive() {
        assert_eq!(subtract(5.0, 3.0), 2.0);
    }

    #[test]
    fn test_subtract_negative() {
        assert_eq!(subtract(-5.0, -3.0), -2.0);
    }

    #[test]
    fn test_subtract_zero() {
        assert_eq!(subtract(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_subtract_mixed() {
        assert_eq!(subtract(-2.0, 3.0), -5.0);
    }

    #[test]
    fn test_multiply_positive() {
        assert_eq!(multiply(2.0, 3.0), 6.0);
    }

    #[test]
    fn test_multiply_negative() {
        assert_eq!(multiply(-2.0, -3.0), 6.0);
    }

    #[test]
    fn test_multiply_zero() {
        assert_eq!(multiply(5.0, 0.0), 0.0);
    }

    #[test]
    fn test_multiply_mixed() {
        assert_eq!(multiply(-2.0, 3.0), -6.0);
    }

    #[test]
    fn test_divide_positive() {
        assert_eq!(divide(6.0, 3.0).unwrap(), 2.0);
    }

    #[test]
    fn test_divide_negative() {
        assert_eq!(divide(-6.0, -3.0).unwrap(), 2.0);
    }

    #[test]
    fn test_divide_mixed() {
        assert_eq!(divide(-6.0, 3.0).unwrap(), -2.0);
    }

    #[test]
    fn test_divide_fractional() {
        assert_eq!(divide(7.0, 2.0).unwrap(), 3.5);
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(5.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Cannot divide by zero");
    }
}

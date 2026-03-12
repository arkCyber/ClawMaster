//! Math operations tool for Moltis AI agents.

wit_bindgen::generate!({
    world: "math-ops",
    exports: {
        world: MathOpsTool,
    },
});

struct MathOpsTool;

impl Guest for MathOpsTool {
    fn add(a: f64, b: f64) -> f64 {
        a + b
    }
    
    fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }
    
    fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }
    
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            return Err("Division by zero".to_string());
        }
        Ok(a / b)
    }
    
    fn modulo(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            return Err("Modulo by zero".to_string());
        }
        Ok(a % b)
    }
    
    fn power(base: f64, exponent: f64) -> f64 {
        base.powf(exponent)
    }
    
    fn sqrt(n: f64) -> Result<f64, String> {
        if n < 0.0 {
            return Err("Cannot compute square root of negative number".to_string());
        }
        Ok(n.sqrt())
    }
    
    fn abs(n: f64) -> f64 {
        n.abs()
    }
    
    fn floor(n: f64) -> f64 {
        n.floor()
    }
    
    fn ceil(n: f64) -> f64 {
        n.ceil()
    }
    
    fn round(n: f64) -> f64 {
        n.round()
    }
    
    fn min(a: f64, b: f64) -> f64 {
        a.min(b)
    }
    
    fn max(a: f64, b: f64) -> f64 {
        a.max(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(MathOpsTool::add(2.0, 3.0), 5.0);
        assert_eq!(MathOpsTool::add(-1.0, 1.0), 0.0);
    }
    
    #[test]
    fn test_subtract() {
        assert_eq!(MathOpsTool::subtract(5.0, 3.0), 2.0);
        assert_eq!(MathOpsTool::subtract(3.0, 5.0), -2.0);
    }
    
    #[test]
    fn test_multiply() {
        assert_eq!(MathOpsTool::multiply(2.0, 3.0), 6.0);
        assert_eq!(MathOpsTool::multiply(-2.0, 3.0), -6.0);
    }
    
    #[test]
    fn test_divide() {
        assert_eq!(MathOpsTool::divide(6.0, 2.0).unwrap(), 3.0);
        assert!(MathOpsTool::divide(1.0, 0.0).is_err());
    }
    
    #[test]
    fn test_modulo() {
        assert_eq!(MathOpsTool::modulo(7.0, 3.0).unwrap(), 1.0);
        assert!(MathOpsTool::modulo(1.0, 0.0).is_err());
    }
    
    #[test]
    fn test_power() {
        assert_eq!(MathOpsTool::power(2.0, 3.0), 8.0);
        assert_eq!(MathOpsTool::power(5.0, 2.0), 25.0);
    }
    
    #[test]
    fn test_sqrt() {
        assert_eq!(MathOpsTool::sqrt(9.0).unwrap(), 3.0);
        assert_eq!(MathOpsTool::sqrt(16.0).unwrap(), 4.0);
        assert!(MathOpsTool::sqrt(-1.0).is_err());
    }
    
    #[test]
    fn test_abs() {
        assert_eq!(MathOpsTool::abs(-5.0), 5.0);
        assert_eq!(MathOpsTool::abs(5.0), 5.0);
    }
    
    #[test]
    fn test_floor() {
        assert_eq!(MathOpsTool::floor(3.7), 3.0);
        assert_eq!(MathOpsTool::floor(-3.7), -4.0);
    }
    
    #[test]
    fn test_ceil() {
        assert_eq!(MathOpsTool::ceil(3.2), 4.0);
        assert_eq!(MathOpsTool::ceil(-3.2), -3.0);
    }
    
    #[test]
    fn test_round() {
        assert_eq!(MathOpsTool::round(3.5), 4.0);
        assert_eq!(MathOpsTool::round(3.4), 3.0);
    }
    
    #[test]
    fn test_min() {
        assert_eq!(MathOpsTool::min(3.0, 5.0), 3.0);
        assert_eq!(MathOpsTool::min(5.0, 3.0), 3.0);
    }
    
    #[test]
    fn test_max() {
        assert_eq!(MathOpsTool::max(3.0, 5.0), 5.0);
        assert_eq!(MathOpsTool::max(5.0, 3.0), 5.0);
    }
}

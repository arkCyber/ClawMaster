//! Math statistics tool for Moltis AI agents.

wit_bindgen::generate!({
    world: "math-stats",
    exports: {
        world: MathStatsTool,
    },
});

struct MathStatsTool;

impl Guest for MathStatsTool {
    fn mean(numbers: Vec<f64>) -> Result<f64, String> {
        if numbers.is_empty() {
            return Err("Empty array".to_string());
        }
        Ok(numbers.iter().sum::<f64>() / numbers.len() as f64)
    }
    
    fn median(mut numbers: Vec<f64>) -> Result<f64, String> {
        if numbers.is_empty() {
            return Err("Empty array".to_string());
        }
        
        numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = numbers.len();
        
        if len % 2 == 0 {
            Ok((numbers[len / 2 - 1] + numbers[len / 2]) / 2.0)
        } else {
            Ok(numbers[len / 2])
        }
    }
    
    fn std_dev(numbers: Vec<f64>) -> Result<f64, String> {
        let variance = Self::variance(numbers)?;
        Ok(variance.sqrt())
    }
    
    fn variance(numbers: Vec<f64>) -> Result<f64, String> {
        if numbers.is_empty() {
            return Err("Empty array".to_string());
        }
        
        let mean = Self::mean(numbers.clone())?;
        let variance = numbers.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / numbers.len() as f64;
        
        Ok(variance)
    }
    
    fn min(numbers: Vec<f64>) -> Result<f64, String> {
        numbers.iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .ok_or_else(|| "Empty array".to_string())
    }
    
    fn max(numbers: Vec<f64>) -> Result<f64, String> {
        numbers.iter()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .ok_or_else(|| "Empty array".to_string())
    }
    
    fn sum(numbers: Vec<f64>) -> f64 {
        numbers.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mean() {
        assert_eq!(MathStatsTool::mean(vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap(), 3.0);
    }
    
    #[test]
    fn test_median() {
        assert_eq!(MathStatsTool::median(vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap(), 3.0);
        assert_eq!(MathStatsTool::median(vec![1.0, 2.0, 3.0, 4.0]).unwrap(), 2.5);
    }
    
    #[test]
    fn test_std_dev() {
        let result = MathStatsTool::std_dev(vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_variance() {
        let result = MathStatsTool::variance(vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_min() {
        assert_eq!(MathStatsTool::min(vec![3.0, 1.0, 4.0, 1.0, 5.0]).unwrap(), 1.0);
    }
    
    #[test]
    fn test_max() {
        assert_eq!(MathStatsTool::max(vec![3.0, 1.0, 4.0, 1.0, 5.0]).unwrap(), 5.0);
    }
    
    #[test]
    fn test_sum() {
        assert_eq!(MathStatsTool::sum(vec![1.0, 2.0, 3.0, 4.0, 5.0]), 15.0);
    }
}

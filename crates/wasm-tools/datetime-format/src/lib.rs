//! DateTime formatting tool for Moltis AI agents.

use time::{OffsetDateTime, format_description};

wit_bindgen::generate!({
    world: "datetime-format",
    exports: {
        world: DateTimeFormatTool,
    },
});

struct DateTimeFormatTool;

impl Guest for DateTimeFormatTool {
    fn format_timestamp(timestamp: i64, format: String) -> Result<String, String> {
        let dt = OffsetDateTime::from_unix_timestamp(timestamp)
            .map_err(|e| format!("Invalid timestamp: {}", e))?;
        
        let fmt = format_description::parse(&format)
            .map_err(|e| format!("Invalid format: {}", e))?;
        
        dt.format(&fmt)
            .map_err(|e| format!("Format error: {}", e))
    }
    
    fn format_iso(iso: String, format: String) -> Result<String, String> {
        let dt = OffsetDateTime::parse(&iso, &time::format_description::well_known::Iso8601::DEFAULT)
            .map_err(|e| format!("Invalid ISO8601: {}", e))?;
        
        let fmt = format_description::parse(&format)
            .map_err(|e| format!("Invalid format: {}", e))?;
        
        dt.format(&fmt)
            .map_err(|e| format!("Format error: {}", e))
    }
    
    fn parse_datetime(input: String, format: String) -> Result<i64, String> {
        let fmt = format_description::parse(&format)
            .map_err(|e| format!("Invalid format: {}", e))?;
        
        let dt = OffsetDateTime::parse(&input, &fmt)
            .map_err(|e| format!("Parse error: {}", e))?;
        
        Ok(dt.unix_timestamp())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_timestamp() {
        let result = DateTimeFormatTool::format_timestamp(1609459200, "[year]-[month]-[day]".to_string());
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_format_iso() {
        let result = DateTimeFormatTool::format_iso("2021-01-01T00:00:00Z".to_string(), "[year]-[month]-[day]".to_string());
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_parse_datetime() {
        let result = DateTimeFormatTool::parse_datetime("2021-01-01".to_string(), "[year]-[month]-[day]".to_string());
        assert!(result.is_ok());
    }
}

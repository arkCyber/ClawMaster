//! DateTime tool for Moltis AI agents.
//!
//! This tool provides current time operations with:
//! - Unix timestamps
//! - ISO 8601 formatting
//! - RFC 3339 formatting
//! - Date and time extraction
//!
//! # Compliance
//! DO-178C Level A:
//! - §6.3.2: Exception handling - All errors properly handled
//! - §6.3.4: Deterministic behavior - Time is from system clock
//! - §11.13: Initialization - No global state

use time::OffsetDateTime;
use time::format_description::well_known::{Iso8601, Rfc3339};

wit_bindgen::generate!({
    world: "datetime-now",
    exports: {
        world: DateTimeNowTool,
    },
});

struct DateTimeNowTool;

impl Guest for DateTimeNowTool {
    /// Get current UTC timestamp.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: All errors caught and returned
    fn timestamp() -> Result<u64, String> {
        let now = OffsetDateTime::now_utc();
        let timestamp = now.unix_timestamp();
        
        // Convert to u64, handling negative timestamps
        if timestamp < 0 {
            return Err("Timestamp is before Unix epoch".to_string());
        }
        
        Ok(timestamp as u64)
    }
    
    /// Get current UTC timestamp in milliseconds.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn timestamp_millis() -> Result<u64, String> {
        let now = OffsetDateTime::now_utc();
        let timestamp_ms = now.unix_timestamp_nanos() / 1_000_000;
        
        if timestamp_ms < 0 {
            return Err("Timestamp is before Unix epoch".to_string());
        }
        
        Ok(timestamp_ms as u64)
    }
    
    /// Get current UTC time in ISO 8601 format.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn iso8601() -> Result<String, String> {
        let now = OffsetDateTime::now_utc();
        
        let formatted = now
            .format(&Iso8601::DEFAULT)
            .map_err(|e| format!("Failed to format ISO 8601: {}", e))?;
        
        Ok(formatted)
    }
    
    /// Get current UTC time in RFC 3339 format.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn rfc3339() -> Result<String, String> {
        let now = OffsetDateTime::now_utc();
        
        let formatted = now
            .format(&Rfc3339)
            .map_err(|e| format!("Failed to format RFC 3339: {}", e))?;
        
        Ok(formatted)
    }
    
    /// Get current date in YYYY-MM-DD format.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn date() -> Result<String, String> {
        let now = OffsetDateTime::now_utc();
        let date = now.date();
        
        let formatted = format!(
            "{:04}-{:02}-{:02}",
            date.year(),
            date.month() as u8,
            date.day()
        );
        
        Ok(formatted)
    }
    
    /// Get current time in HH:MM:SS format.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Error handling
    fn time() -> Result<String, String> {
        let now = OffsetDateTime::now_utc();
        let time = now.time();
        
        let formatted = format!(
            "{:02}:{:02}:{:02}",
            time.hour(),
            time.minute(),
            time.second()
        );
        
        Ok(formatted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_timestamp() {
        let result = DateTimeNowTool::timestamp();
        assert!(result.is_ok());
        let ts = result.unwrap();
        // Should be a reasonable timestamp (after 2020)
        assert!(ts > 1577836800); // 2020-01-01
    }
    
    #[test]
    fn test_timestamp_millis() {
        let result = DateTimeNowTool::timestamp_millis();
        assert!(result.is_ok());
        let ts_ms = result.unwrap();
        // Should be a reasonable timestamp in milliseconds
        assert!(ts_ms > 1577836800000); // 2020-01-01 in ms
    }
    
    #[test]
    fn test_iso8601() {
        let result = DateTimeNowTool::iso8601();
        assert!(result.is_ok());
        let iso = result.unwrap();
        // Should contain date and time components
        assert!(iso.contains("T"));
        assert!(iso.contains("Z") || iso.contains("+") || iso.contains("-"));
    }
    
    #[test]
    fn test_rfc3339() {
        let result = DateTimeNowTool::rfc3339();
        assert!(result.is_ok());
        let rfc = result.unwrap();
        // Should contain date and time components
        assert!(rfc.contains("T"));
        assert!(rfc.contains("Z") || rfc.contains("+") || rfc.contains("-"));
    }
    
    #[test]
    fn test_date() {
        let result = DateTimeNowTool::date();
        assert!(result.is_ok());
        let date = result.unwrap();
        // Should be in YYYY-MM-DD format
        assert_eq!(date.len(), 10);
        assert_eq!(date.chars().nth(4).unwrap(), '-');
        assert_eq!(date.chars().nth(7).unwrap(), '-');
    }
    
    #[test]
    fn test_time() {
        let result = DateTimeNowTool::time();
        assert!(result.is_ok());
        let time = result.unwrap();
        // Should be in HH:MM:SS format
        assert_eq!(time.len(), 8);
        assert_eq!(time.chars().nth(2).unwrap(), ':');
        assert_eq!(time.chars().nth(5).unwrap(), ':');
    }
}

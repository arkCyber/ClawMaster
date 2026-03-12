//! CSV parsing functionality.

use crate::{ParserError, Result};
use serde_json::Value;
use std::io::Read;

/// CSV parser with various output formats.
pub struct CsvParser {
    delimiter: u8,
    has_headers: bool,
}

impl Default for CsvParser {
    fn default() -> Self {
        Self::new()
    }
}

impl CsvParser {
    /// Create a new CSV parser with default settings.
    pub fn new() -> Self {
        Self {
            delimiter: b',',
            has_headers: true,
        }
    }
    
    /// Set the delimiter character.
    pub fn with_delimiter(mut self, delimiter: u8) -> Self {
        self.delimiter = delimiter;
        self
    }
    
    /// Set whether the CSV has headers.
    pub fn with_headers(mut self, has_headers: bool) -> Self {
        self.has_headers = has_headers;
        self
    }
    
    /// Parse CSV string to JSON array.
    pub fn parse_to_json(&self, input: &str) -> Result<String> {
        let records = self.parse_to_records(input)?;
        let json = serde_json::to_string(&records)?;
        Ok(json)
    }
    
    /// Parse CSV string to JSON array (pretty printed).
    pub fn parse_to_json_pretty(&self, input: &str) -> Result<String> {
        let records = self.parse_to_records(input)?;
        let json = serde_json::to_string_pretty(&records)?;
        Ok(json)
    }
    
    /// Parse CSV to vector of records.
    fn parse_to_records(&self, input: &str) -> Result<Vec<Value>> {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(self.delimiter)
            .has_headers(self.has_headers)
            .from_reader(input.as_bytes());
        
        let headers = if self.has_headers {
            Some(reader.headers()?.clone())
        } else {
            None
        };
        
        let mut records = Vec::new();
        
        for result in reader.records() {
            let record = result?;
            
            let obj = if let Some(ref headers) = headers {
                // Create object with headers as keys
                let mut map = serde_json::Map::new();
                for (i, field) in record.iter().enumerate() {
                    if let Some(header) = headers.get(i) {
                        map.insert(header.to_string(), Value::String(field.to_string()));
                    }
                }
                Value::Object(map)
            } else {
                // Create array
                let array: Vec<Value> = record.iter()
                    .map(|f| Value::String(f.to_string()))
                    .collect();
                Value::Array(array)
            };
            
            records.push(obj);
        }
        
        Ok(records)
    }
    
    /// Get headers from CSV.
    pub fn get_headers(&self, input: &str) -> Result<Vec<String>> {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(self.delimiter)
            .has_headers(self.has_headers)
            .from_reader(input.as_bytes());
        
        if self.has_headers {
            let headers = reader.headers()?;
            Ok(headers.iter().map(|h| h.to_string()).collect())
        } else {
            Err(ParserError::InvalidInput("CSV has no headers".to_string()))
        }
    }
    
    /// Validate CSV format.
    pub fn validate(&self, input: &str) -> Result<bool> {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(self.delimiter)
            .has_headers(self.has_headers)
            .from_reader(input.as_bytes());
        
        let mut count = 0;
        for result in reader.records() {
            result?;
            count += 1;
        }
        
        Ok(count > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_to_json() {
        let csv = "name,age\nAlice,30\nBob,25";
        let parser = CsvParser::new();
        let result = parser.parse_to_json(csv).unwrap();
        assert!(result.contains("Alice"));
        assert!(result.contains("30"));
    }
    
    #[test]
    fn test_get_headers() {
        let csv = "name,age,city\nAlice,30,NYC";
        let parser = CsvParser::new();
        let headers = parser.get_headers(csv).unwrap();
        assert_eq!(headers, vec!["name", "age", "city"]);
    }
    
    #[test]
    fn test_custom_delimiter() {
        let csv = "name;age\nAlice;30";
        let parser = CsvParser::new().with_delimiter(b';');
        let result = parser.parse_to_json(csv).unwrap();
        assert!(result.contains("Alice"));
    }
    
    #[test]
    fn test_validate() {
        let csv = "name,age\nAlice,30";
        let parser = CsvParser::new();
        assert!(parser.validate(csv).unwrap());
    }
}

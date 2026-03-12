//! XML parsing functionality.

use crate::{ParserError, Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use serde_json::{json, Value};

/// XML parser with JSON output.
pub struct XmlParser;

impl XmlParser {
    /// Create a new XML parser.
    pub fn new() -> Self {
        Self
    }
    
    /// Parse XML string to JSON.
    pub fn parse_to_json(&self, input: &str) -> Result<String> {
        let value = self.parse_to_value(input)?;
        let json = serde_json::to_string(&value)?;
        Ok(json)
    }
    
    /// Parse XML string to JSON (pretty printed).
    pub fn parse_to_json_pretty(&self, input: &str) -> Result<String> {
        let value = self.parse_to_value(input)?;
        let json = serde_json::to_string_pretty(&value)?;
        Ok(json)
    }
    
    /// Parse XML to serde_json::Value.
    fn parse_to_value(&self, input: &str) -> Result<Value> {
        let mut reader = Reader::from_str(input);
        reader.trim_text(true);
        
        let mut buf = Vec::new();
        let mut stack: Vec<(String, serde_json::Map<String, Value>)> = Vec::new();
        let mut current_text = String::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    stack.push((name, serde_json::Map::new()));
                    current_text.clear();
                }
                Ok(Event::End(_)) => {
                    if let Some((name, mut map)) = stack.pop() {
                        if !current_text.trim().is_empty() {
                            map.insert("_text".to_string(), Value::String(current_text.trim().to_string()));
                        }
                        
                        let value = if map.is_empty() {
                            Value::String(current_text.trim().to_string())
                        } else {
                            Value::Object(map)
                        };
                        
                        if let Some((_, parent_map)) = stack.last_mut() {
                            parent_map.insert(name, value);
                        } else {
                            // Root element
                            let mut root = serde_json::Map::new();
                            root.insert(name, value);
                            return Ok(Value::Object(root));
                        }
                        
                        current_text.clear();
                    }
                }
                Ok(Event::Text(e)) => {
                    current_text.push_str(&e.unescape().map_err(|e| ParserError::Xml(e.to_string()))?);
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(ParserError::Xml(format!("XML parse error: {}", e))),
                _ => {}
            }
            buf.clear();
        }
        
        Ok(json!({}))
    }
    
    /// Validate XML format.
    pub fn validate(&self, input: &str) -> Result<bool> {
        let mut reader = Reader::from_str(input);
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Eof) => break,
                Err(e) => return Err(ParserError::Xml(format!("Invalid XML: {}", e))),
                _ => {}
            }
            buf.clear();
        }
        
        Ok(true)
    }
    
    /// Get root element name.
    pub fn get_root_element(&self, input: &str) -> Result<String> {
        let mut reader = Reader::from_str(input);
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    return Ok(name);
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(ParserError::Xml(format!("XML parse error: {}", e))),
                _ => {}
            }
            buf.clear();
        }
        
        Err(ParserError::InvalidInput("No root element found".to_string()))
    }
}

impl Default for XmlParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_to_json() {
        let xml = r#"<root><name>Alice</name><age>30</age></root>"#;
        let parser = XmlParser::new();
        let result = parser.parse_to_json(xml).unwrap();
        assert!(result.contains("Alice"));
    }
    
    #[test]
    fn test_validate() {
        let xml = r#"<root><item>test</item></root>"#;
        let parser = XmlParser::new();
        assert!(parser.validate(xml).unwrap());
    }
    
    #[test]
    fn test_get_root_element() {
        let xml = r#"<root><item>test</item></root>"#;
        let parser = XmlParser::new();
        let root = parser.get_root_element(xml).unwrap();
        assert_eq!(root, "root");
    }
}

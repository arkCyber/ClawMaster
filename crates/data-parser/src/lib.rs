//! Data parsing library for Moltis AI agents.
//! 
//! This crate provides native Rust implementations of data parsing
//! functionality that is not feasible in Wasm environments due to
//! complex dependencies or size constraints.

pub mod csv;
pub mod xml;
pub mod yaml;

pub use csv::CsvParser;
pub use xml::XmlParser;
pub use yaml::YamlParser;

/// Common error type for all parsers.
#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("CSV parsing error: {0}")]
    Csv(#[from] csv::Error),
    
    #[error("XML parsing error: {0}")]
    Xml(String),
    
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, ParserError>;

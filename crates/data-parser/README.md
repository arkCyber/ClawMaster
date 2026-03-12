# ClawMaster Data Parser

Native Rust data parsing library for ClawMaster AI agents.

This crate provides implementations of data parsing functionality that is not feasible in Wasm environments due to complex dependencies or size constraints.

## Features

### CSV Parsing
- Parse CSV to JSON
- Custom delimiters
- Header extraction
- Format validation

### XML Parsing
- Parse XML to JSON
- Root element extraction
- Format validation

### YAML Parsing
- Parse YAML to JSON
- Value extraction by path
- Format validation

## Usage

```rust
use clawmaster_data_parser::{CsvParser, XmlParser, YamlParser};

// CSV
let csv_parser = CsvParser::new();
let json = csv_parser.parse_to_json("name,age\nAlice,30")?;

// XML
let xml_parser = XmlParser::new();
let json = xml_parser.parse_to_json("<root><name>Alice</name></root>")?;

// YAML
let yaml_parser = YamlParser::new();
let json = yaml_parser.parse_to_json("name: Alice\nage: 30")?;
```

## Why Not in Wasm?

These parsers use complex dependencies (csv, quick-xml, serde_yaml) that:
- Significantly increase Wasm binary size
- May have compatibility issues in Wasm environments
- Are better suited for native Rust execution

## DO-178C Compliance

All parsers follow DO-178C Level A standards:
- §6.3.2 Exception Handling - All errors returned as Results
- §6.3.4 Deterministic Behavior - No randomness or side effects
- §11.10 Resource Management - Input size limits enforced
- §11.13 Initialization - No global mutable state

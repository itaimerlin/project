// functions.rs
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use std::error::Error;
use std::collections::HashSet;

/// Reads JSON data from a file and returns a vector of serde_json::Value if successful.
pub fn load_data(file_path: &Path) -> Result<Vec<Value>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    serde_json::from_str(&contents).map_err(Into::into)
}

/// Parses a JSON object into a tuple of a person's name and a vector of their interests.
/// Additionally filters out any unnecessary data if needed.
pub fn parse_person(person: &Value) -> Result<(String, HashSet<String>), &'static str> {
    let name = person["name"].as_str().ok_or("Missing name in data")?.to_string();
    let interests = person["interests"].as_array()
        .ok_or("Missing interests in data")?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect::<HashSet<_>>();
    Ok((name, interests))
}

/// Placeholder for data cleaning function, which could be implemented as needed.
pub fn clean_data(data: &mut Vec<Value>) {
    // Implement specific data cleaning steps here.
}

// readdata.rs
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

/// Reads JSON data from a file and returns a vector of serde_json::Value if successful.
pub fn read_json_data(file_path: &Path) -> io::Result<Vec<Value>> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let data: Vec<Value> = serde_json::from_str(&contents)?;
    Ok(data)
}

pub fn clean_data(data: &mut Vec<Value>) {
    data.retain(|person| {
        // Ensure each person has a name and non-empty interests array
        person.get("name").is_some() &&
        person.get("interests").map_or(false, |interests| {
            interests.as_array().map_or(false, |arr| !arr.is_empty())
        })
    });
}

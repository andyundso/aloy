use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize,Debug)]
pub struct Configuration {
    paths: Vec<String>
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Configuration, Box<dyn Error>> {
  // Open the file in read-only mode with buffer.
  let file = File::open(path)?;
  let reader = BufReader::new(file);

  // Read the JSON contents of the file as an instance of `Configuration`.
  let u = serde_json::from_reader(reader)?;

  // Return the `Configuration`.
  Ok(u)
}

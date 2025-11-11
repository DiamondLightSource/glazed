use std::fs::File;
use std::io::BufReader;

use serde::de::DeserializeOwned;

pub fn assert_readable_as<T: DeserializeOwned>(path: &str) {
    let file = File::open(path).unwrap();
    let rdr = BufReader::new(file);
    serde_json::from_reader::<_, T>(rdr).unwrap();
}

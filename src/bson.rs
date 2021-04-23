use serde_json::{Map, Value};
use bson::Document;
use std::error;

pub fn to_doc(value: &str) -> Result<Document, Box<dyn error::Error>> {

  let v: Map<String, Value> = match serde_json::from_str(value) {
    Ok(val) => {
      val
    },
    Err(e) => {
      return Err(Box::new(e))
    }
  };

  match bson::to_document(&v) {
    Ok(d) => Ok(d),
    Err(e) => {
      eprintln!("Error converting to bson", e);
      return Err(Box::new(e))
    }
  }
}

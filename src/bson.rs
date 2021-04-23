use serde_json::{Map, Value};
use bson::Document;
use std::error;
use bson::de::from_bson;

pub fn to_doc(value: &str) -> Result<Document, Box<dyn error::Error>> {

  let v: Map<String, Value> = match serde_json::from_str(value) {
    Ok(val) => {
      val
    },
    Err(e) => {
      eprintln!("Error data converting to json: {}", e);
      return Err(Box::new(e))
    }
  };

  match Document::try_from(v) {
    Ok(d) => Ok(d),
    Err(e) => {
      return Err(Box::new(e))
    }
  }
}

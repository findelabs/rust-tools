use serde_json::{Map, Value};
use bson::Document;

pub fn to_doc(string: &str) -> Result<Document, Box<dyn error::Error>> {

  let v: Map<String, Value> = match serde_json::from_str(value) {
    Ok(val) => {
      val
    },
    Err(e) => {
      return Err(e)
    }
  };

  let data = match bson::to_document(&v) {
    Ok(d) => d,
    Err(e) => {
      return Err(e)
    }
  };
}

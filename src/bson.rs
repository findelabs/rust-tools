use serde_json::{Map, Value};
use bson::Document;
use std::error;
use std::convert::TryFrom;

pub fn to_doc(value: &str) -> Result<Document, Box<dyn error::Error + Send + Sync>> {

  let v: Map<String, Value> = match serde_json::from_str(value) {
    Ok(val) => {
      val
    },
    Err(e) => {
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

pub fn vec_to_doc(value: &str) -> Result<Vec<Document>, Box<dyn error::Error + Send + Sync>> {

  let v: Vec<Map<String, Value>> = match serde_json::from_str(value) {
    Ok(val) => {
      val
    },
    Err(e) => {
      return Err(Box::new(e))
    }
  };

  let mut result: Vec<Document> = Vec::new();
  for doc in v {
    match Document::try_from(doc) {
      Ok(d) => result.push(d),
      Err(e) => {
        return Err(Box::new(e))
      }
    }
  }
  Ok(result)
}

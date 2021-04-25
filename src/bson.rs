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

pub fn to_doc_vec(value: &str) -> Result<Vec<Document>, Box<dyn error::Error + Send + Sync>> {

  let v: Vec<Map<String, Value>> = match serde_json::from_str(value) {
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

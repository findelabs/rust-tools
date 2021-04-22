use serde_json::{Map, Value};
use bson::Document;

pub fn get_root_path(string: &str) -> Result<Document> {

  let v: Map<String, Value> = match serde_json::from_str(value) {
    Ok(val) => {
      log::info!("Successfully transformed data into json");
      val
    },
    Err(e) => {
      log::info!("Got error converting data to json {}", e);
      return Err(e)
    }
  };

  let data = match bson::to_document(&v) {
    Ok(d) => d,
    Err(e) => {
      log::info!("Got error converting {} to bson: {}", value, e);
      return Err(e)
    }
  };
}

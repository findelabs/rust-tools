use hyper::Request;
http::request::Parts;

pub fn get_root_path(parts: &Parts) -> Option<String> {
  match parts.uri.path() {
    "/" => "default".to_owned(),
    _ => {
      let stage_one: Vec<&str> = parts.uri.path().split("/").collect();    // Convert to array
      let stage_two = &stage_one[1..stage_one.len() - 1];                  // Remove the last path
      let stage_three = stage_two.join("_");                               // Join back with underscores
      stage_three
    }
  }
}

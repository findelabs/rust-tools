use http::request::Parts;
use url::form_urlencoded;
use std::collections::HashMap;

pub type Queries = HashMap<String, String>;
 
pub fn queries(parts: &Parts) -> Option<Queries> {
    let queries: HashMap<String, String> = parts
        .uri
        .query()
        .map(|v| {
            form_urlencoded::parse(v.as_bytes())
            .into_owned()
            .collect()
        })
        .unwrap_or_else(HashMap::new);
    
    Some(queries)
}

use http::request::Parts;
use url::form_urlencoded;
use std::collections::HashMap;
use std::error::Error;

//type Result<T> = Result<T,Box<dyn Error + Send + Sync>>;
pub type Queries = HashMap<String, String>;

// let _queries = queries(&parts).expect("Failed to generate hashmap of queries");
pub fn queries(parts: &Parts) -> Result<Queries,Box<dyn Error + Send + Sync>> {
    let queries: HashMap<String, String> = parts
        .uri
        .query()
        .map(|v| {
            form_urlencoded::parse(v.as_bytes())
            .into_owned()
            .collect()
        })
        .unwrap_or_else(HashMap::new);
    
    Ok(queries)
}

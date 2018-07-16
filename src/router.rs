use std::cmp;
use regex::Regex;
use stdweb::web::document;
use stdweb::web::error::SecurityError;
 use url::percent_encoding::percent_decode;

#[derive(Debug)]
pub struct Router<R> {
    routes: Vec<R>
}

impl<R: cmp::PartialEq> Router<R> {

    pub fn new() -> Router<R> {
      Router{
          routes: Vec::new()
      }
    }

    pub fn add(&mut self, route: R) {
        self.routes.push(route)
    }

    pub fn remove(&mut self, route: R) {
        let index = self.routes.iter().position(|r| *r == route).unwrap();
        self.routes.remove(index);
    }

    pub fn get_fragment(&self) -> Result<String, SecurityError> {
        let re = Regex::new(r"\?(.*)$").unwrap();
        self.get_fragmented_url().map(|fragment| {
            self.clear_slashes(&Self::decode(&re.replace_all(&fragment, "").to_string()))
        })
    }

    fn get_fragmented_url(&self) -> Result<String, SecurityError> {
        if let Some(location) = document().location() {
            location.pathname().and_then(|pathname| {
                location.search().map(|search| [pathname, search].join(""))
            })
        } else {
            Ok("/".to_string())
        }
    }

    pub fn decode(uri: &str) -> String {
        percent_decode(uri.as_bytes()).decode_utf8().unwrap().to_string()
    }

    fn clear_slashes(&self, path: &str) -> String {
        let re1 = Regex::new(r"\\/$").unwrap();
        let first_path = re1.replace_all(path, "").to_string();
        let re2 = Regex::new(r"^\\/").unwrap();
        re2.replace_all(&first_path, "").to_string()
    }
}

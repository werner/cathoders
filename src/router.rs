use std::{cmp, fmt};
use regex;
use regex::Regex;
use stdweb::web::IEventTarget;
use stdweb::web::document;
use stdweb::web::window;
use stdweb::JsSerialize;
use stdweb::web::event::PopStateEvent;
use stdweb::web::error::SecurityError;
use url::percent_encoding::percent_decode;

#[derive(Debug)]
pub struct Router<R> {
    routes: Vec<R>
}

impl<R: cmp::PartialEq + fmt::Display + JsSerialize> Router<R> {

    pub fn new() -> Router<R> {
      Router{
          routes: Vec::new()
      }
    }

    pub fn add(&mut self, route: R) -> Result<(), regex::Error> {
        match self.route_as_regex(&route) {
            Ok(_) => Ok(self.routes.push(route)),
            Err(error) => Err(error)
        }
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

    pub fn listen(&'static self) {
        window().add_event_listener( move |_: PopStateEvent| {
            self.check(None);
        });
    }

    pub fn navigate(&self, route: &R) {
        window().history().push_state(route, &route.to_string(), Some(&self.clear_slashes(&route.to_string())))
    }

    fn check(&self, optFragment: Option<String>) -> Result<bool, SecurityError> {
        self.get_fragment().map(|fragment| {
            let fragment_to_comp = optFragment.unwrap_or(fragment);
            self.routes.iter().any(|route| {
                let re = self.route_as_regex(route);
                // I'm making sure it's a valid regex when adding to routes
                re.unwrap().is_match(&fragment_to_comp)
            })
        })
    }

    fn route_as_regex(&self, route: &R) -> Result<Regex, regex::Error> {
        Regex::new(&format!(r"{}", regex::escape(&route.to_string())))
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

    fn decode(uri: &str) -> String {
        percent_decode(uri.as_bytes()).decode_utf8().unwrap().to_string()
    }

    fn clear_slashes(&self, path: &str) -> String {
        let re1 = Regex::new(r"\\/$").unwrap();
        let first_path = re1.replace_all(path, "").to_string();
        let re2 = Regex::new(r"^\\/").unwrap();
        re2.replace_all(&first_path, "").to_string()
    }
}

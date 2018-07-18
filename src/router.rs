use std::{fmt};
use std::cmp::PartialEq;
use regex;
use regex::Regex;
use stdweb::web::IEventTarget;
use stdweb::web::document;
use stdweb::web::window;
use stdweb::JsSerialize;
use stdweb::web::event::PopStateEvent;
use stdweb::web::error::SecurityError;
use url::percent_encoding::percent_decode;

trait Component { }

#[derive(Debug, PartialEq)]
struct Route<S, P> {
    state: S,
    path: P
}

#[derive(Debug)]
pub struct Router<S, P> {
    routes: Vec<Route<S, P>>
}

impl<S: JsSerialize, P: PartialEq + fmt::Display> Router<S, P> {

    pub fn new() -> Router<S, P> {
      Router{
          routes: Vec::new()
      }
    }

    pub fn add(&mut self, state: S, path: P) -> Result<(), regex::Error> {
        let route = Route { state, path };
        match self.route_as_regex(&route) {
            Ok(_) => Ok(self.routes.push(route)),
            Err(error) => Err(error)
        }
    }

    pub fn remove(&mut self, path: P) {
        let index = self.routes.iter().position(|r| r.path == path).unwrap();
        self.routes.remove(index);
    }

    pub fn get_fragment(&self) -> Result<String, SecurityError> {
        let re = Regex::new(r"\?(.*)$").unwrap();
        self.get_fragmented_url().map(|fragment| {
            self.clear_slashes(&Self::decode(&re.replace_all(&fragment, "").to_string()))
        })
    }

    pub fn navigate(&self, state: S, path: P) {
        window().history().push_state(state,
                                      &path.to_string(),
                                      Some(&self.clear_slashes(&path.to_string())))
    }

    pub fn check(&self, optFragment: Option<String>) -> Result<bool, SecurityError> {
        self.get_fragment().map(|fragment| {
            let fragment_to_comp = optFragment.unwrap_or(fragment);
            self.routes.iter().any(|route| {
                let re = self.route_as_regex(route);
                // I'm making sure it's a valid regex when adding to routes
                re.unwrap().is_match(&fragment_to_comp)
            })
        })
    }

    fn route_as_regex(&self, route: &Route<S, P>) -> Result<Regex, regex::Error> {
        Regex::new(&format!(r"{}", regex::escape(&route.path.to_string())))
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

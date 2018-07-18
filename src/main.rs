#[macro_use] extern crate stdweb;
extern crate regex;
extern crate url;

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod router;

use router::Router;
use stdweb::web::window;
use stdweb::web::IEventTarget;
use stdweb::web::event::PopStateEvent;

#[derive(Serialize, Deserialize, Debug)]
struct Product(String);

js_serializable!(Product);

fn main() {
    stdweb::initialize();
    let mut router: Router<Product, String> = Router::new();
    router.add(Product("products".to_string()), "products".to_string());
    router.add(Product("products".to_string()), "to_delete".to_string());
    router.remove("to_delete".to_string());
    window().add_event_listener( move |_: PopStateEvent| {
        println!("pop state");
    });
    println!("{:?}", router.get_fragment());
    println!("{:?}", router);
    router.navigate(Product("products".to_string()), "products".to_string());
    stdweb::initialize();
}

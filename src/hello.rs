extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate handlebars_iron as hbs;

use std::str::FromStr;
use std::env;
use iron::prelude::*;
use router::Router;
use iron::status;
use std::path::Path;
use staticfile::Static;
use mount::Mount;
use hbs::{Template, HandlebarsEngine, DirectorySource, MemorySource};
use std::collections::BTreeMap;
use std::sync::Arc;

// Serves a string to the user.  Try accessing "/".
fn hello(_: &mut Request) -> IronResult<Response> {
    let resp = Response::with((status::Ok, "Hello world !!!"));
    Ok(resp)
}

// Serves a customized string to the user.  Try accessing "/world".
fn hello_name(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let name = params.find("name").unwrap();

    let mut resp = Response::new();
    let mut datatree = BTreeMap::new();
    datatree.insert("name".to_string(), name.to_string());
    resp.set_mut(Template::new("hello", datatree)).set_mut(status::Ok);
    Ok(resp)
}

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(8080)
}

/// Configure and run our server.
// based on https://medium.com/@ericdreichert/rusts-iron-framework-getting-started-b90977811d8c
fn main() {
    // set up handlebars views
    let mut hbse = HandlebarsEngine::new();
    let views_path = "./views/";
    hbse.add(Box::new(DirectorySource::new(views_path, ".hbs")));
    if let Err(r) = hbse.reload() {
        panic!("views failed");
    }
    let hbse_ref = Arc::new(hbse);
    // hbse_ref.watch(views_path);

    // set up Chain for hello_name
    let mut hello_chain = Chain::new(hello_name);
    hello_chain.link_after(hbse_ref);

    // Set up our URL router.
    let mut router: Router = Router::new();
    router.get("/", hello, "index");
    router.get("/name/:name", hello_chain, "name");

    // static directory
    let mut mount = Mount::new();
    mount.mount("/", router)
         .mount("/static/", Static::new(Path::new("./public/")));

    // Run the server.
    Iron::new(mount).http(("0.0.0.0", get_server_port())).unwrap();
}

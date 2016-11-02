extern crate iron;
extern crate router;
extern crate mount;
extern crate params;
extern crate staticfile;

#[macro_use]
extern crate json;

use std::path::Path;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;

use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::AfterMiddleware;
use router::{Router, NoRoute};
use mount::Mount;
use params::Params;
use staticfile::Static;

const HOST: &'static str = "localhost:4000";

struct CatchErrs;

impl AfterMiddleware for CatchErrs {
    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        println!("catching errors");

        if let Some(_) = err.error.downcast::<NoRoute>() {
            println!("   Some");
            let mut resp = Response::with((status::InternalServerError, format!("<h1>404</h1><pre>{:?}\n{}</pre>", req, err)));
            resp.headers.set(ContentType("text/html".parse().unwrap()));
            Ok(resp)
        } else {
            println!("   Error: {:#?}", err);
            let mut resp = Response::with((status::InternalServerError, format!("<h1>500</h1><pre>{:?}\n{}</pre>", req, err)));
            resp.headers.set(ContentType("text/html".parse().unwrap()));
            Ok(resp)
            // Err(err)
        }
    }
}

fn serve_id(req: &mut Request) -> IronResult<Response> {
    let id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .and_then(|id| id.parse::<i32>().ok());
    if let Some(id) = id {
        Ok(Response::with((status::Ok, format!("you wanted ID:{:?}", id))))
    } else {
        // Ok(Response::with((status::NotFound, format!("cannot interpret {:?} as id", id))))
        Err(IronError::new(Error::new(ErrorKind::Other, "oh no!"),
                           format!("cannot interpret {:?} as id", id)))
    }
}

fn post_print(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<Params>().unwrap();
    println!("map:\n{:?}", map);
    Ok(Response::with((status::Ok, format!("you posted {:?}", map))))
}

fn post_shout(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<Params>().unwrap();
    let mut new_map = HashMap::new();

    for (k, v) in map.iter() {
        if let &params::Value::String(ref v) = v {
            new_map.insert(k, v.to_uppercase());
        }
    }

    Ok(Response::with((status::Ok, format!("you posted {:?}", new_map))))
}

fn show_routes(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, format!("{:?}", stringify!(not implemented sorry)))))
}

fn setup_static_server() {
    println!("http listening on: http://{}", HOST);

    let content_path = Path::new("./htdocs/");

    let mut router = Router::new();
    router.get("/", Static::new(content_path), "index");
    router.get("/", Static::new(content_path), "index");
    router.get("/*", Static::new(content_path), "content");
    router.get("/routes", show_routes, "routes");
    router.get("/routes/", show_routes, "routes/");
    router.get("/items/:id", serve_id, "items");
    router.post("/print_name", post_print, "post_print");
    router.post("/shout_name", post_shout, "post_shout");

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/images/", Static::new("./htdocs/assets/images/"));
    mount.mount("/docs/", Static::new("target/doc"));


    let mut chain = Chain::new(mount);
    chain.link_after(CatchErrs);

    Iron::new(chain).http(HOST).unwrap();
}


fn main() {
    setup_static_server();
}

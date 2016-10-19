extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate websocket;

#[macro_use]
extern crate json;

use std::path::Path;
use std::io::{Error, ErrorKind};
use std::time;
use std::thread;
use std::collections::HashMap;
use std::io::Read;

use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::AfterMiddleware;
use router::{Router, NoRoute, url_for};
use mount::Mount;
use staticfile::Static;

use websocket::{Server, Message};

const HOST: &'static str = "localhost:3000";
const WS_HOST: &'static str = "localhost:3001";

struct CatchErrs;

impl AfterMiddleware for CatchErrs {
    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        println!("catching errors");

        if let Some(_) = err.error.downcast::<NoRoute>() {
            println!("   Some");
            let mut resp = Response::with((status::InternalServerError,
                                           format!("<h1>404</h1><pre>{:?}\n{}</pre>", req, err)));
            resp.headers.set(ContentType("text/html".parse().unwrap()));
            Ok(resp)
        } else {
            println!("   Error: {:?}", err);
            let mut resp = Response::with((status::InternalServerError,
                                           format!("<h1>500</h1><pre>{:?}\n{}</pre>", req, err)));
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

fn show_routes(req: &mut Request) -> IronResult<Response> {
    Ok(
        Response::with(
            (status::Ok,
             format!("{:?}", stringify!(foo)
                    )
            )
            )
      )
}

fn setup_static_server() {
    println!("http listening on: http://{}", HOST);

    let content_path = Path::new("content/");
    let assets_path = Path::new("assets/");

    let mut router = Router::new();
    router.get("/", Static::new(content_path), "index");
    router.get("/routes", show_routes, "routes");
    router.get("/routes/", show_routes, "routes/");
    //router.get("/*", Static::new(content_path), "content");
    router.get("/images/*", Static::new(assets_path), "images");
    router.get("/items/:id", serve_id, "items");

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/docs/", Static::new("target/doc"));
    mount.mount("/favicon.ico",
                Static::new(Path::new("assets/images/favicon.png")));

    let mut chain = Chain::new(mount);
    chain.link_after(CatchErrs);

    Iron::new(chain).http(HOST).unwrap();
}

fn setup_websocket_server() {
    println!("websocket listening on: http://{}", WS_HOST);

    let ws_server = Server::bind(WS_HOST).unwrap();
    for connection in ws_server {
        thread::spawn(move || {
            let request = connection.unwrap().read_request().unwrap(); // Get the request
            let response = request.accept(); // Form a response

            let mut client = response.send().unwrap(); // Send the response


            loop {
                let now = time::SystemTime::now();
                let message = Message::text(json::stringify(object!{
                    "timestamp" => format!("{:?}",now),
                    "color"  => object!{
                        "R" => 0,
                        "G" => 0,
                        "B" => 255
                    },
                    "answer" => 42


                        }));
                let _ = client.send_message(&message);
                thread::sleep(time::Duration::from_millis(5000));
            }
        });
    }
}

fn main() {
    //thread::spawn(|| setup_websocket_server());
    setup_static_server();
}

#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::header::{HeaderValue, ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_ORIGIN};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

static INDEX1: &[u8] = b"Hello, world!";

async fn index1(_: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut body: Response<Body> = Response::new(Body::from(INDEX1));

    let headers = body.headers_mut();

    headers.append(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );

    headers.append(
        ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("https://defcronyke.github.io"),
    );

    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let mut port: u16 = 8080;
    match std::env::var("PORT") {
        Ok(p) => {
            match p.parse::<u16>() {
                Ok(n) => {
                    port = n;
                }
                Err(_e) => {}
            };
        }
        Err(_e) => {}
    };

    let addr1 = ([0, 0, 0, 0], port).into();

    let srv1 = Server::bind(&addr1).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(index1))
    }));

    println!("Listening on http://{}", addr1);

    let _ret = srv1.await;

    Ok(())
}

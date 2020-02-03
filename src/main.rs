use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Client};
use hyper::service::{make_service_fn, service_fn};
use env_logger;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let req_url = _req.uri();
    let proxy_url = &format!("http://172.17.0.1:8080{}?{}", req_url.path(), (
        match req_url.query() {
            Some(v) => v,
            None => "",
        }
    ));

    let mut req = Request::builder()
        .method(_req.method())
        .uri(proxy_url);

    for (k, v) in _req.headers() {
        if k != "HOST" && k != "LOCATION" {
            req = req.header(k, v);
        }
    }
    let body = _req.into_body();
    let request = req.body(body).unwrap();

    let result = Client::new().request(request).await.unwrap();

    Ok(result)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    // We'll bind to 0.0.0.0:5000
    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

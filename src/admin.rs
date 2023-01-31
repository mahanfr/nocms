use std::{error::Error};
use std::convert::Infallible;
use std::net::SocketAddr;
use http_body::Full;
use hyper::body::Bytes;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use crate::utils::get_file;

async fn admin_service(req: Request<Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    if req.uri() == "/admin" {
        return Ok(
            get_file("public/index.html").await.unwrap()
        )
    }else if req.uri().path().starts_with("/") && req.uri().path().split('.').count() == 2 {
        return Ok(
            get_file(format!("public{}",req.uri().path()).as_str()).await.unwrap()
        );
    }
    else {
        Ok(Response::new("APi Stuff".into()))
    }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

pub async fn run_admin_service() -> Result<(),Box<dyn Error>> {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(admin_service))
    });

    let server = Server::bind(&addr).serve(make_svc);
    let server = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    println!("Admin Server Closed Sucessfully");
    Ok(())
}
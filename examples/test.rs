use bytes::Bytes;
use http_body_util::Full;
use hyper::{body::Incoming, server::conn::http1, Request, Response};
use routerify::{RequestServiceBuilder, Router};
use routerify_query::{query_parser, RequestQueryExt};
use std::{convert::Infallible, net::SocketAddr};
use tokio::net::TcpListener;

async fn home_handler(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let q = req.query("bookname");
    dbg!(q);

    Ok(Response::new(Full::from("Home page")))
}

fn router() -> Router<Incoming, Full<Bytes>, Infallible> {
    Router::builder()
        .middleware(query_parser())
        .get("/", home_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router = router();

    let builder = RequestServiceBuilder::new(router).unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let service = builder.build();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(stream, service).await {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

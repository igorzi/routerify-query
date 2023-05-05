use bytes::Bytes;
use http_body_util::Full;
use hyper::{body::Incoming, server::conn::http1, Request, Response};
use routerify::{RequestServiceBuilder, Router};
// Import the query_parser function and the RequestQueryExt trait.
use routerify_query::{query_parser, RequestQueryExt};
use std::{convert::Infallible, net::SocketAddr};
use tokio::net::TcpListener;

// A handler for "/" page. Visit: "/?username=Alice&bookname=HarryPotter" to see query values.
async fn home_handler(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    // Access the query values.
    let user_name = req.query("username").unwrap();
    let book_name = req.query("bookname").unwrap();

    Ok(Response::new(Full::from(format!(
        "User: {}, Book: {}",
        user_name, book_name
    ))))
}

// Create a router.
fn router() -> Router<Incoming, Full<Bytes>, Infallible> {
    Router::builder()
        // Attach the query_parser middleware.
        .middleware(query_parser())
        .get("/", home_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router = router();

    // Create a Service builder from the router above to handle incoming requests.
    let builder = RequestServiceBuilder::new(router).unwrap();

    // The address on which the server will be listening.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // Create a TcpListener and bind it to the address.
    let listener = TcpListener::bind(addr).await.unwrap();

    // Start a loop to continuously accept incoming connections.
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

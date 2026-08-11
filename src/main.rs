use std::convert::Infallible;

use axum::{
    Router,
    body::Bytes,
    response::{Sse, sse::Event},
    routing::any,
};
use futures_util::{Stream, stream};
use tokio::net::TcpListener;
use tokio_stream::StreamExt as _;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", any(sse_handler));
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

const MESSAGE: &str = include_str!("gigantic.txt");

#[axum::debug_handler]
async fn sse_handler(_body: Bytes) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| Event::default().data(MESSAGE)).map(Ok);
    Sse::new(stream)
}

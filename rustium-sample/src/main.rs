use futures::channel;
use hyper::client::connect::HttpConnector;
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use rustium_api::rustium;
use tokio::runtime::Builder;
use tokio_compat_02::FutureExt;
use warp::Filter;

async fn get_posts_json(
    http_client: Client<HttpsConnector<HttpConnector>, Body>,
    author: String
) -> Result<impl warp::Reply, warp::Rejection> {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    let (sender, receiver) = channel::oneshot::channel::<Result<rustium::RSSFeed, _>>();

    rt.spawn(async move {
        let posts_json = rustium::get_posts(&http_client, author)
            .compat()
            .await;
            sender.send(posts_json).unwrap()
    });

    match receiver.await.unwrap() {
        Ok(value) => Ok(warp::reply::json(&value)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

#[tokio::main]
async fn main() {
    let https_connector = hyper_tls::HttpsConnector::new();
    let https_client = hyper::Client::builder().build::<_, hyper::Body>(https_connector);

    let client_filter = warp::any().map(move || https_client.clone());

    let test = warp::get()
        .and(warp::path("posts"))
        .and(client_filter.clone())
        .and(warp::path::param())
        .and_then(get_posts_json);

    warp::serve(test).run(([127, 0, 0, 1], 3030)).compat().await;
}

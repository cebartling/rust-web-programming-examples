use warp::Filter;

#[tokio::main]
async fn main() {
    // create a path Filter
    let hello = warp::path("hello").map(|| format!("Hello, World!"));

    // start the server and pass the route filter to it
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}

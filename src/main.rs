mod filters;

use warp::Filter;

#[tokio::main]
async fn main() {
    let hi = warp::path("hi").and(warp::get()).map(|| "hello from hi");
    let apis = hi.or(filters::filters_main());


    let routes = apis;

    println!("Hello, world!");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

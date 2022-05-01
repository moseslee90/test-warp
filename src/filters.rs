use warp::Filter;

pub fn filters_main() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("todos")
        .and(warp::get())
        .and(warp::path::end())
        .map(|| "will get todos")
}
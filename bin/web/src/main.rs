use warp::Filter;

#[tokio::main]
async fn main() {
    let serve_files = warp::get().and(warp::fs::dir("/app/bin/web/public"));

    warp::serve(serve_files).run(([0, 0, 0, 0], 80)).await;
}

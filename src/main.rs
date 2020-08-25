use std::env;

#[tokio::main]
async fn main() {

    let args : Vec<String> = env::args().collect();
    let path = args[1].clone();

    let port = 3030;

    println!("Serving {} on {}", path, port);

    warp::serve(warp::fs::dir(path))
        .run(([127, 0, 0, 1], port))
        .await;
}
use std::env;
use std::str::FromStr;

#[tokio::main]
async fn main() {

    let args : Vec<String> = env::args().collect();
    let path = args[1].clone();

    let port: u16 = if (args.len() > 2) { get_port(&args[2]) } else { 3030 };

    println!("Serving {} on {}", path, port);

    warp::serve(warp::fs::dir(path))
        .run(([127, 0, 0, 1], port))
        .await;
}

fn get_port(port_str: &str) -> u16 {
    let port = u16::from_str(port_str).unwrap();

    return port;
}
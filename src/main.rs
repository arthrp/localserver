#![allow(unused_parens)]
use std::env;
use std::str::FromStr;
use std::path::Path;

#[tokio::main]
async fn main() {

    let args : Vec<String> = env::args().collect();
    if(args.len() < 2){
        print_usage();
        return;
    }

    let path = args[1].clone();

    if(!Path::new(&path).exists()) {
        println!("Path {} doesn't exist", path);
        return;
    }

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

fn print_usage() -> () {
    println!("Usage: localserver <path> <port>");
}
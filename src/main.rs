#![allow(unused_parens)]
use std::env;
use std::str::FromStr;
use std::path::Path;
use std::fs;

#[tokio::main]
async fn main() {

    let args : Vec<String> = env::args().collect();
    if(args.len() < 2){
        print_usage();
        return;
    }

    let path_str = args[1].clone();
    let path = Path::new(&path_str);

    if(!path.exists()) {
        println!("Path {} doesn't exist", path_str);
        return;
    }

    let port: u16 = if (args.len() > 2) { get_port(&args[2]) } else { 3030 };

    let abs_path = fs::canonicalize(path).unwrap();
    println!("Serving {:?} on {}", abs_path, port);

    warp::serve(warp::fs::dir(path_str))
        .run(([127, 0, 0, 1], port))
        .await;
}

fn get_port(port_str: &str) -> u16 {
    let port = u16::from_str(port_str).unwrap();

    return port;
}

fn print_usage() -> () {
    println!("Usage: localserver [path] [port]");
}
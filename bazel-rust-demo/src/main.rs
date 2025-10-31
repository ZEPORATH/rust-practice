use reqwest::blocking::get;
use std::env;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: wget_demo <URL>");
        std::process::exit(1);
    }

    let url = &args[1];
    println!("Fetching {}", url);

    match get(url) {
        Ok(mut resp) => {
            println!("Status: {}", resp.status());
            let mut body = vec![0; 200];
            let n = resp.read(&mut body).unwrap_or(0);
            println!("Body (first {} bytes):", n);
            println!("{}", String::from_utf8_lossy(&body[..n]));
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

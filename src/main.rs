use server::Server;
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        println!("Usage: {} [host] [port] [directory]", args[0]);
        println!("Defaults:");
        println!("  host: 127.0.0.1");
        println!("  port: 8787");
        println!("  directory: html");
        std::process::exit(0);
    }

    let host = args.get(1).map_or("127.0.0.1", |s| s);
    let port = args.get(2).map_or("8787", |s| s);
    let dir_name = args.get(3).map_or("html", |s| s);

    // Get binary location and construct html path
    let binary_path = env::current_exe().expect("Failed to get binary path");
    let binary_dir = binary_path.parent().expect("Failed to get binary directory");
    let html_dir = binary_dir.join(dir_name);

    let server = Server::new(5);
    println!("Server started at http://{}:{}", host, port);
    println!("Serving files from: {}", html_dir.display());
    server.start(host, port, &html_dir.to_string_lossy());
}

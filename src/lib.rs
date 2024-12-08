mod file_processor;
mod thread_pool;

use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use file_processor::FileProcessor;
pub use thread_pool::*;

pub struct Server {
    thread_pool: ThreadPool,
}

impl Server {
    pub fn new(thread_num: usize) -> Self {
        let thread_pool = ThreadPool::new(thread_num);
        Self { thread_pool }
    }
    pub fn start(&self, host: &str, port: &str, dir_path: &str) {
        let url = format!("{}:{}", host, port);
        let listener = TcpListener::bind(url).expect("Can't create tcp listener");
        for stream in listener.incoming() {
            let stream = stream.expect("Coulnd't get stream");
            let dir_path = String::from(dir_path);
            self.thread_pool.execute(move || {
                let file_processor = FileProcessor::new(dir_path.clone());
                Server::handle_connection(stream, file_processor, dir_path.clone());
            });
        }
    }

    fn handle_connection(mut stream: TcpStream, file_processor: FileProcessor, dir_path: String) {
        let buf_reader = BufReader::new(&stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();
        
        let route = request_line.split(' ')
            .nth(1)
            .unwrap_or("/");

        let (status_line, file_name): (String, String) = file_processor
            .files
            .iter()
            .find(|file| {
                if route == "/" {
                    file.is_empty()
                } else {
                    **file == &route[1..]
                }
            })
            .map_or(
                (
                    String::from("HTTP/1.1 404 NOT FOUND"),
                    format!("{}/404.html", dir_path),
                ),
                |file| {
                    let file_path = if file.is_empty() {
                        format!("{}/index.html", dir_path)
                    } else {
                        format!("{}/{}.html", dir_path, file)
                    };
                    (String::from("HTTP/1.1 200 OK"), file_path)
                },
            );

        let content = fs::read_to_string(file_name).expect("Couldn't read file");

        let length = content.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

        stream
            .write(response.as_bytes())
            .expect("Couldn't write response to stream");
    }
}

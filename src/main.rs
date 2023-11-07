use std::{
  net::{TcpListener, TcpStream}, 
  io::{BufReader, prelude::*}, 
  fs, thread, 
  time::Duration
};

use sienna::ThreadPool;
fn main() {
  const LOCAL_HOST: &str = "127.0.0.1:7878";

  let listener = TcpListener::bind(LOCAL_HOST).unwrap();
  let tp = ThreadPool::spin(4);

  
  for stream in listener.incoming().take(2) {
    let stream = stream.unwrap();
    tp.execute(|| handle_connection(stream));
  }
}

fn handle_connection(mut stream: TcpStream) {
  let buf = BufReader::new(&mut stream);
  let req = buf.lines().next().unwrap().unwrap();
  
  let (status, template) = match &req[..] {
    "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
    "GET /sleep HTTP/1.1" => {
      thread::sleep(Duration::from_secs(5));
      ("HTTP/1.1 200 OK", "index.html")
    },
    _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
  };

  let contents = fs::read_to_string(template).unwrap();
  let len = contents.len();
  let res = format!("{status}\r\nContent-Length: {len}\r\n\r\n{contents}");

  stream.write_all(res.as_bytes()).unwrap();
}
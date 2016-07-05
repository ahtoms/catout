use std::net::TcpStream;
use std::io::Write;

pub struct CatIn {
    pub stream: TcpStream,
}

impl CatIn {

    pub fn new(stream: TcpStream) -> CatIn {
        return CatIn { stream: stream };
    }

    pub fn write(&mut self, message: &String) -> usize {
        let mut bytes: usize = 0;
        match self.stream.write(message.as_bytes()) {
            Ok(b) => {
                bytes = b;
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        return bytes;
    }
}

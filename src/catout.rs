
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::io::Write;
use std::str;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

pub struct CatOut {
    clients: Arc<Mutex<Vec<Box<CatIn>>>>,
}

pub struct CatIn {
    stream: TcpStream,
}

impl CatIn {

    pub fn new(stream: TcpStream) -> CatIn {
        return CatIn { stream: stream };
    }

    pub fn write(&mut self, message: &String) -> usize {
        let bytes = self.stream.write(message.as_bytes()).unwrap();
        return bytes;
    }
}

impl CatOut {

    pub fn new() -> CatOut {
        return CatOut { clients: Arc::new(Mutex::new(Vec::new())) };
    }

    pub fn listen(&mut self, bind_address: &str) {
        let listener = TcpListener::bind(bind_address).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let mut clients = self.clients.lock().unwrap();
                    let ref client;
                    clients.push(Box::new(CatIn::new(stream)));
                    client = clients.last_mut().unwrap();
                    CatOut::handle_client(client);
                }
                Err(e) => { println!("{}", e); }
            }
        }
    }

    pub fn start_program_monitor(&mut self, program_name: &str, arguments: Vec<String>) {
        let process;
        let mut command = Command::new(program_name);

        for arg in arguments {
            command.arg(&arg[..]);
        }

        command.stderr(Stdio::piped());
        command.stdout(Stdio::piped());
        process = command.spawn().unwrap();

        let mut proc_stdout = process.stdout.unwrap();
        let mut proc_stderr = process.stderr.unwrap();
        let clients_clone_stdout = self.clients.clone();
        let clients_clone_stderr = self.clients.clone();

        thread::spawn(move || {
            let mut line = String::new();
            let clients = clients_clone_stdout;
            loop {
                match proc_stdout.read_to_string(&mut line) {
                    Ok(n) => {
                        for c in clients.lock().unwrap().iter_mut() {
                            c.write(&line);
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        });

        thread::spawn(move || {
            let mut line = String::new();
            let clients = clients_clone_stderr;
            loop {
                match proc_stderr.read_to_string(&mut line) {
                    Ok(n) => {
                        for c in clients.lock().unwrap().iter_mut() {
                            c.write(&line);
                        }
                    }
                    Err(e) => {
                        println!("error: {}", e);
                    }
                }
            }
        });
    }

    pub fn handle_client(client: & Box<CatIn>) {
        let mut stream = client.stream.try_clone().unwrap();
        thread::spawn(move || {
            loop {
                let mut buf: [u8; 256] = [0; 256];
                if stream.read(&mut buf).unwrap() > 0 {
                    print!("{}", str::from_utf8(&buf).unwrap());
                } else {
                    println!("Client has terminated");
                    break;
                }
            }
        });
    }
}

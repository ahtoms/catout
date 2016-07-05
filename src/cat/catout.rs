use std::net::TcpListener;
use std::net::Shutdown;
use std::thread;
use std::io::Read;
use std::io::Write;
use std::str;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

use cat::catin::CatIn;

macro_rules! buffer_size { () => (8192) }

///
pub struct CatOut {
    clients: Arc<Mutex<Vec<Box<CatIn>>>>,
}

impl CatOut {

    /// Creates a new CatOut object
    ///
    pub fn new() -> CatOut {
        return CatOut { clients: Arc::new(Mutex::new(Vec::new())) };
    }

    ///Listens for incoming connections
    pub fn listen(&mut self, bind_address: &str) {
        let listener = TcpListener::bind(bind_address).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let mut clients = self.clients.lock().unwrap();
                    clients.push(Box::new(CatIn::new(stream)));
                    let ref client = clients.last_mut().unwrap();
                    CatOut::handle_client(client);
                }
                Err(e) => { println!("{}", e); }
            }
        }
    }

    pub fn handle_client(client: & Box<CatIn>) {
        let mut stream = client.stream.try_clone().unwrap();
        thread::spawn(move || {
            loop {
                let mut buf: [u8; buffer_size!()] = [0; buffer_size!()];
                match stream.read(&mut buf) {
                    Ok(_) => {
                        //unwrap() on
                        match str::from_utf8(&buf) {
                            Ok(_) => { }
                            Err(e) => { /*println!("Error: {}", e);*/  break; }
                        }
                    }
                    Err(e) => { /*println!("Error: {}", e);*/ break; }
                }
            }
            match stream.shutdown(Shutdown::Both) {
                Ok(_) => { }
                Err(e) => { /*println!("Error: {}", e);*/  }
            }
        });
    }

    pub fn start_program(&mut self, program_name: &str, arguments: Vec<String>) {
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
            let mut buf: [u8; buffer_size!()] = [0; buffer_size!()];
            let clients = clients_clone_stdout;
            loop {
                let mut removal_list: Vec<usize> = Vec::new();
                let mut index: usize = 0;
                match proc_stdout.read(&mut buf) {
                    Ok(_) => {
                        let mut client_lock = clients.lock().unwrap();
                        for c in client_lock.iter_mut() {
                            let string = String::from_utf8_lossy(&buf).into_owned();
                            if c.write(&string) == 0 {
                                removal_list.push(index);
                            }
                            index = index + 1;
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
                for i in removal_list {
                    clients.lock().unwrap().swap_remove(i);
                }
            }
        });

        thread::spawn(move || {
            let mut buf: [u8; buffer_size!()] = [0; buffer_size!()];
            let clients = clients_clone_stderr;
            loop {
                match proc_stderr.read(&mut buf) {
                    Ok(_) => {
                        for c in clients.lock().unwrap().iter_mut() {
                            let string = String::from_utf8_lossy(&buf).into_owned();
                            c.write(&string);
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        });
    }
}

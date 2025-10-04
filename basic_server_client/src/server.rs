use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct Server{
    pub addr: String,
}

impl Server {
    pub fn New(addr:&str) -> Self {
        Self {addr: addr.to_string()}
    }

    pub fn WaitForClient(&self) -> std::io::Result<()>{
        let listener = TcpListener::bind(self.addr.clone());
        println!("Server listening on {}", self.addr);

        for stream_res in listener?.incoming() {
            match stream_res {
                Ok(stream) => {
                    thread::spawn(move || {
                        if let Err(e) = Self::handle_client(stream) {
                            eprintln!("client handler failed: {}", e);
                        }
                    });
                }
                Err(e) => eprintln!("accept error: {}", e),
            }
        }
        Ok(())
    }

    fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
        let peer = stream.peer_addr().ok();
        println!("peer connecting at: {:?}", peer);

        let mut reader = BufReader::new(stream.try_clone()?);
        let mut line = String::new();

        loop {
            line.clear();
            let bytes = reader.read_line(&mut line);
            if bytes? == 0 {
                println!("Client {:?}disconnected!", peer);
                break;
            }
            let message = line.trim_end().to_string();
            println!("recieved from {:?}: {}", peer, message);

            stream.write_all(message.as_bytes());
            stream.write_all(b"\n");
        }
        Ok(())
    }
    
}

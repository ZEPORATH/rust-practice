use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

pub struct Client {
    pub addr: String,
}

impl Client {
    pub fn New(addr: &str) -> Self {
        Self { addr: addr.to_string() }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let stream = TcpStream::connect(&self.addr)?;
        println!("connected to :{}", self.addr);

        let mut read_stream = stream.try_clone()?;
        let mut write_stream = stream;

        // spawn the reader thread
        thread::spawn(move || {
            let reader = BufReader::new(read_stream);
            for line in reader.lines() {
                match line {
                    Ok(l) => println!("recieved : {}", l),
                    Err(e) => {
                        eprintln!("error in reading: {}", e);
                        break
                    }
                }
            }
            println!("server connection closed!");
        });
        
        // main input thread
        let stdin = io::stdin();
        for line in stdin.lines() {
            let line  = line?; 
            writeln!(write_stream, "{}", line)?;
            write_stream.flush();
        }
        Ok(())
    }
}
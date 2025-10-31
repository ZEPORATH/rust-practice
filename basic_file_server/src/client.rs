use async_std::io::{self, prelude::*, BufReader};
use async_std::net::TcpStream;
use clap::Parser;
use md5::Context;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Async non-blocking client with progress", long_about = None)]
pub struct ClientCli {
    /// server address, e.g. 127.0.0.1:4000
    #[arg(short, long)]
    pub addr: String,

    /// filename to GET; if omitted, client will list and prompt
    #[arg(short, long)]
    pub get: Option<String>,

    /// output directory
    #[arg(short, long)]
    pub out: Option<PathBuf>,
}

pub struct Client {
    addr: String,
}

impl Client {
    pub fn new(addr: &str) -> Self {
        Self { addr: addr.to_string() }
    }

    pub async fn run_cli(cli: ClientCli) -> io::Result<()> {
        let client = Client::new(&cli.addr);
        let mut stream = TcpStream::connect(&client.addr).await?;
        println!("Connected to {}", client.addr);
        client.list_and_get(&mut stream, cli.get, cli.out).await
    }

    pub async fn list_and_get(&self, stream: &mut TcpStream, get_filename: Option<String>, out_dir: Option<PathBuf>) -> io::Result<()> {
        println!("listing and getting");
        // ask for LIST
        writeln!(stream, "{}", "LIST").await?;
        stream.flush().await?;
        println!("LIST sent");
        
        let mut reader = BufReader::new(stream.clone());
        let mut filenames = Vec::new();
        loop {
            let mut line = String::new();
            let n = reader.read_line(&mut line).await?;
            if n == 0 { return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "server closed")); }
            let line = line.trim_end();
            if line == "." { break; }
            println!("- {}", line);
            filenames.push(line.to_string());
        }

        let filename = match get_filename {
            Some(f) => f,
            None => {
                println!("Pick a file to GET (type exact name):");
                let mut s = String::new();
                io::stdin().read_line(&mut s).await?;
                s.trim().to_string()
            }
        };

        // Send GET using the same stream
        let cmd = format!("GET {}
", filename);
        stream.write_all(cmd.as_bytes()).await?;
        stream.flush().await?;

        // We'll use the reader to parse the FILE header
        let mut header = String::new();
        reader.read_line(&mut header).await?;
        if header.starts_with("ERR") {
            println!("Server error: {}", header);
            return Ok(());
        }
        if !header.starts_with("FILE ") {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("unexpected header: {}", header)));
        }
        let size_str = header[5..].trim();
        let size: usize = size_str.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        println!("Receiving {} bytes...", size);

        // Create output file
        let out_dir = out_dir.unwrap_or_else(|| PathBuf::from("."));
        let mut file = File::create(out_dir.join(&filename))?;

        // Read exactly size bytes in chunks and show progress
        let mut remaining = size;
        let mut buf = vec![0u8; 64 * 1024];  // 64KB buffer
        let mut total_read = 0usize;
        let mut context = Context::new();

        while remaining > 0 {
            let to_read = std::cmp::min(buf.len(), remaining);
            let n = reader.read(&mut buf[..to_read]).await?;
            if n == 0 { 
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, 
                    format!("server closed while sending file (got {}/{} bytes)", total_read, size))); 
            }
            context.consume(&buf[..n]);
            file.write_all(&buf[..n])?;
            total_read += n;
            remaining -= n;
            
            // Show progress every 1MB or at the end
            if total_read % (1024 * 1024) == 0 || remaining == 0 {
                print!("\rDownloaded {}/{} bytes ({:.1}%)", 
                    total_read, size, (total_read as f64 / size as f64) * 100.0);
                use std::io::Write as _;
                std::io::stdout().flush()?;
            }
        }
        println!();

        // read trailing newline
        let mut nl = [0u8; 1];
        reader.read_exact(&mut nl).await?;

        // read MD5 line
        let mut md5_line = String::new();
        reader.read_line(&mut md5_line).await?;
        if !md5_line.starts_with("MD5 ") {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("expected MD5, got: {}", md5_line)));
        }
        let md5_hex = md5_line[4..].trim();

        let our_hex = format!("{:x}", context.finalize());
        if our_hex == md5_hex {
            println!("MD5 OK: {}", md5_hex);
        } else {
            println!("MD5 MISMATCH! server: {} local: {}", md5_hex, our_hex);
        }

        Ok(())
    }
}

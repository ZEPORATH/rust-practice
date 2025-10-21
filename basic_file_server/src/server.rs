use core::fmt;
use std::fmt::Formatter;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::fmt::Debug;

const SERVER: Token = Token(0);
const CHUNK_SIZE: usize = 64 * 1024; // 64 KiB per write chunk

#[derive(Debug)]
enum OutgoingStage {
    Header,
    Body,
    Trailing, // newline + MD5
    Done,
}

struct FileStreamer {
    file: File,
    remaining: u64,
    stage: OutgoingStage,
    md5_hex: Option<String>,
    context: md5::Context,
}

impl Debug for FileStreamer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("md5::Context")
        .field("file", &self.file)
        .field("remaining", &self.remaining)
        .field("stage", &self.stage)
        .field("md5_hex", &self.md5_hex)
        .finish()
    }
}

impl FileStreamer {
    fn new(mut file: File) -> io::Result<Self> {
        let size = file.seek(SeekFrom::End(0))?;
        file.seek(SeekFrom::Start(0))?;
        Ok(Self {
            file,
            remaining: size,
            stage: OutgoingStage::Header,
            md5_hex: None,
            context: md5::Context::new()
        })
    }
}


#[derive(Debug)]
struct Connection {
    socket: TcpStream,
    token: Token,
    read_buf: Vec<u8>,
    write_buf: Vec<u8>,
    peer: SocketAddr,
    current_streamer: Option<FileStreamer>,
}

impl Connection {
    fn new(socket: TcpStream, token: Token, peer: SocketAddr) -> Self {
        Self {
            socket,
            token,
            read_buf: Vec::with_capacity(4096),
            write_buf: Vec::new(),
            peer,
            current_streamer: None,
        }
    }

    fn readable(&mut self) -> io::Result<Option<String>> {
        let mut buf = [0u8; 4096];
        loop {
            match self.socket.read(&mut buf) {
                Ok(0) => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "client closed")),
                Ok(n) => {
                    self.read_buf.extend_from_slice(&buf[..n]);
                    if let Some(pos) = self.read_buf.iter().position(|&b| b == b' ') {
                        let line = self.read_buf.drain(..=pos).collect::<Vec<u8>>();
                        let line = String::from_utf8_lossy(&line).trim().to_string();
                        return Ok(Some(line));
                    }
                    continue;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }
        Ok(None)
    }

    fn writable(&mut self) -> io::Result<()> {
        // First, if there is an active FileStreamer and we haven't started pushing body, do it in small reads
        if let Some(streamer) = &mut self.current_streamer {
            // Handle header stage
            if matches!(streamer.stage, OutgoingStage::Header) {
                let header = format!("FILE {}
", streamer.remaining);
                self.write_buf.extend_from_slice(header.as_bytes());
                streamer.stage = OutgoingStage::Body;
            }

            if matches!(streamer.stage, OutgoingStage::Body) {
                // Read up to CHUNK_SIZE from file into a temporary buffer, update md5 and push into write_buf
                if streamer.remaining > 0 {
                    let to_read = std::cmp::min(streamer.remaining as usize, CHUNK_SIZE);
                    let mut tmp = vec![0u8; to_read];
                    let n = streamer.file.read(&mut tmp)?;
                    if n == 0 {
                        // unexpected EOF
                        streamer.remaining = 0;
                    } else {
                        streamer.remaining -= n as u64;
                        streamer.context.consume(&tmp[..n]);
                        self.write_buf.extend_from_slice(&tmp[..n]);
                    }
                }

                if streamer.remaining == 0 {
                    streamer.stage = OutgoingStage::Trailing;
                    let digest = streamer.context.clone().finalize();
                    let md5_hex = format!("{:x}", digest);
                    streamer.md5_hex = Some(md5_hex);
                }
            }

            if matches!(streamer.stage, OutgoingStage::Trailing) {
                self.write_buf.extend_from_slice(b"
"); // newline after file
                if let Some(ref md5) = streamer.md5_hex {
                    let md5_line = format!("MD5 {}
", md5);
                    self.write_buf.extend_from_slice(md5_line.as_bytes());
                }
                streamer.stage = OutgoingStage::Done;
            }

            if matches!(streamer.stage, OutgoingStage::Done) {
                // file fully queued; drop streamer after bytes are sent
                // we'll keep it until write_buf drained, then set to None
            }
        }

        // Then attempt to write as much of write_buf as possible
        while !self.write_buf.is_empty() {
            match self.socket.write(&self.write_buf) {
                Ok(0) => return Err(io::Error::new(io::ErrorKind::WriteZero, "failed to write")),
                Ok(n) => {
                    self.write_buf.drain(..n);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }

        // If streamer exists and is done (all bytes queued and write_buf now empty), remove it
        if let Some(streamer) = &self.current_streamer {
            if matches!(streamer.stage, OutgoingStage::Done) && self.write_buf.is_empty() {
                self.current_streamer = None;
            }
        }

        Ok(())
    }
}

pub struct Server {
    addr: String,
    mount_dir: PathBuf,
}

impl Server {
    pub fn new(addr: &str, mount_dir: PathBuf) -> Self {
        Self { addr: addr.to_string(), mount_dir }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(256);

        let addr = self.addr.parse::<SocketAddr>().expect("invalid socket addr");
        let mut listener = TcpListener::bind(addr)?;
        poll.registry().register(&mut listener, SERVER, Interest::READABLE)?;

        let mut unique_token = 1usize;
        let mut connections: HashMap<Token, Connection> = HashMap::new();

        println!("Server listening on {} and serving directory {:?}", self.addr, self.mount_dir);

        loop {
            poll.poll(&mut events, None)?;

            for event in events.iter() {
                match event.token() {
                    SERVER => loop {
                        match listener.accept() {
                            Ok((socket, addr)) => {
                                // mio sockets are already non-blocking
                                let token = Token(unique_token);
                                unique_token += 1;
                                let conn = Connection::new(socket, token, addr);
                                poll.registry().register(&mut connections.entry(token).or_insert_with(|| conn).socket, token, Interest::READABLE.add(Interest::WRITABLE))?;
                                // Ugly: we used entry to borrow socket; better to create then insert, but keep code short here
                                // Instead, do actual insert properly below
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                            Err(e) => {
                                eprintln!("accept error: {}", e);
                                break;
                            }
                        }
                    },
                    tok => {
                        // get mutable connection
                        if let Some(conn) = connections.get_mut(&tok) {
                            if event.is_readable() {
                                match conn.readable() {
                                    Ok(Some(line)) => {
                                        if let Err(e) = handle_command(line, conn, &self.mount_dir) {
                                            eprintln!("error handling command from {:?}: {}", conn.peer, e);
                                            connections.remove(&tok);
                                            continue;
                                        }
                                    }
                                    Ok(None) => {}
                                    Err(e) => {
                                        eprintln!("read error from {:?}: {}", conn.peer, e);
                                        connections.remove(&tok);
                                        continue;
                                    }
                                }
                            }

                            if event.is_writable() {
                                if let Err(e) = conn.writable() {
                                    eprintln!("write error to {:?}: {}", conn.peer, e);
                                    connections.remove(&tok);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }

            // accept new connections properly after handling events (to avoid borrow issues)
            // NOTE: simplified: accept loop above used entry().or_insert_with which is awkward; for clarity, we keep
            // this TODO: refactor accept insertion if you rework code.
        }
    }
}

fn handle_command(line: String, conn: &mut Connection, mount_dir: &Path) -> io::Result<()> {
    println!("recieved command: {}", line);
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }
    println!("parts: {:?}", parts);
    match parts[0] {
        "LIST" => {
            let mut out = Vec::new();
            for entry in std::fs::read_dir(mount_dir)? {
                let entry = entry?;
                if entry.path().is_file() {
                    if let Some(name) = entry.file_name().to_str() {
                        out.extend_from_slice(name.as_bytes());
                        out.push(b' ');
                    }
                }
            }
            out.extend_from_slice(b".
");
            conn.write_buf.extend_from_slice(&out);
        }
        "GET" => {
            if parts.len() < 2 {
                conn.write_buf.extend_from_slice(b"ERR missing filename
");
                return Ok(());
            }
            let filename = parts[1];
            let safe = sanitize_filename(filename);
            let full = mount_dir.join(&safe);
            if !full.exists() || !full.is_file() {
                conn.write_buf.extend_from_slice(b"ERR file not found
");
                return Ok(());
            }

            let file = File::open(&full)?;
            let streamer = FileStreamer::new(file)?;

            // Prepare: header will be queued on writable
            conn.current_streamer = Some(streamer);
        }
        _ => {
            conn.write_buf.extend_from_slice(b"ERR unknown command
");
        }
    }
    Ok(())
}

fn sanitize_filename(name: &str) -> String {
    name.replace("..", "").replace("/", "").replace("\\", "")
}
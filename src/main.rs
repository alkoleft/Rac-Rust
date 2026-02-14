use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

struct Args {
    listen: String,
    target: String,
    log_dir: PathBuf,
    try_inflate: bool,
}

#[derive(Copy, Clone)]
enum Direction {
    ClientToServer,
    ServerToClient,
}

struct SessionLogger {
    root: PathBuf,
    state: Mutex<LoggerState>,
}

struct LoggerState {
    exchange_id: u64,
    current_exchange: Option<u64>,
    event_id: u64,
}

impl SessionLogger {
    fn new(root: PathBuf) -> io::Result<Self> {
        fs::create_dir_all(root.join("exchanges"))?;
        Ok(Self {
            root,
            state: Mutex::new(LoggerState {
                exchange_id: 0,
                current_exchange: None,
                event_id: 0,
            }),
        })
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn log_chunk(&self, direction: Direction, data: &[u8]) -> io::Result<()> {
        self.append_stream(direction, data)?;
        let exchange_id = self.append_exchange(direction, data)?;
        self.append_event(direction, data, exchange_id)
    }

    fn append_stream(&self, direction: Direction, data: &[u8]) -> io::Result<()> {
        let filename = match direction {
            Direction::ClientToServer => "client_to_server.stream.bin",
            Direction::ServerToClient => "server_to_client.stream.bin",
        };
        append_to_file(self.root.join(filename), data)
    }

    fn append_exchange(&self, direction: Direction, data: &[u8]) -> io::Result<u64> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "logger mutex poisoned"))?;

        let exchange_id = match direction {
            Direction::ClientToServer => {
                state.exchange_id += 1;
                state.current_exchange = Some(state.exchange_id);
                state.exchange_id
            }
            Direction::ServerToClient => state.current_exchange.unwrap_or(0),
        };

        let exchange_dir = self
            .root
            .join("exchanges")
            .join(format!("exchange_{exchange_id:06}"));
        fs::create_dir_all(&exchange_dir)?;

        let file_name = match direction {
            Direction::ClientToServer => "request.bin",
            Direction::ServerToClient => "response.bin",
        };
        append_to_file(exchange_dir.join(file_name), data)?;
        Ok(exchange_id)
    }

    fn append_event(&self, direction: Direction, data: &[u8], exchange_id: u64) -> io::Result<()> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "logger mutex poisoned"))?;
        state.event_id += 1;
        let event_id = state.event_id;
        drop(state);

        let direction_str = match direction {
            Direction::ClientToServer => "c2s",
            Direction::ServerToClient => "s2c",
        };
        let ts_millis = now_unix_millis();
        let preview = hex_preview(data, 32);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.root.join("events.log"))?;
        writeln!(
            file,
            "event_id={event_id} ts_unix_ms={ts_millis} dir={direction_str} exchange_id={exchange_id} bytes={} preview_hex={preview}",
            data.len()
        )?;
        Ok(())
    }
}
fn append_to_file(path: PathBuf, data: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(data)
}

fn now_unix_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn hex_preview(data: &[u8], max_len: usize) -> String {
    let preview_len = data.len().min(max_len);
    let mut s = String::with_capacity(preview_len * 2 + 3);
    for &b in &data[..preview_len] {
        s.push_str(&format!("{b:02x}"));
    }
    if data.len() > preview_len {
        s.push_str("...");
    }
    s
}

fn write_hex_dump_if_exists(input_path: &Path, output_path: &Path) -> io::Result<()> {
    if !input_path.exists() {
        return Ok(());
    }

    let data = fs::read(input_path)?;
    let mut out = File::create(output_path)?;
    writeln!(out, "file={}", input_path.display())?;
    writeln!(out, "size={}", data.len())?;
    writeln!(out)?;
    write!(out, "{}", render_hex_ascii(&data))?;
    Ok(())
}

fn render_hex_ascii(data: &[u8]) -> String {
    let mut out = String::new();
    for (line_index, chunk) in data.chunks(16).enumerate() {
        let offset = line_index * 16;
        out.push_str(&format!("{offset:08x}  "));

        for i in 0..16 {
            if i < chunk.len() {
                out.push_str(&format!("{:02x}", chunk[i]));
            } else {
                out.push_str("  ");
            }

            if i == 7 {
                out.push_str("  ");
            } else {
                out.push(' ');
            }
        }

        out.push_str(" |");
        for &b in chunk {
            let ch = if (0x20..=0x7e).contains(&b) {
                b as char
            } else {
                '.'
            };
            out.push(ch);
        }
        for _ in chunk.len()..16 {
            out.push(' ');
        }
        out.push_str("|\n");
    }
    out
}

fn generate_text_dumps(session_root: &Path) -> io::Result<()> {
    write_hex_dump_if_exists(
        &session_root.join("client_to_server.stream.bin"),
        &session_root.join("client_to_server.stream.hex.txt"),
    )?;
    write_hex_dump_if_exists(
        &session_root.join("server_to_client.stream.bin"),
        &session_root.join("server_to_client.stream.hex.txt"),
    )?;

    let exchanges_dir = session_root.join("exchanges");
    if !exchanges_dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(&exchanges_dir)? {
        let exchange_dir = entry?.path();
        if !exchange_dir.is_dir() {
            continue;
        }

        write_hex_dump_if_exists(
            &exchange_dir.join("request.bin"),
            &exchange_dir.join("request.hex.txt"),
        )?;
        write_hex_dump_if_exists(
            &exchange_dir.join("response.bin"),
            &exchange_dir.join("response.hex.txt"),
        )?;
    }

    Ok(())
}

fn session_directory(base_dir: &Path, client_addr: &str) -> PathBuf {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let pid = std::process::id();
    let sanitized_addr = client_addr.replace([':', '.'], "_");
    base_dir.join(format!("session_{ts}_{pid}_{sanitized_addr}"))
}

fn relay(
    mut reader: TcpStream,
    mut writer: TcpStream,
    logger: Arc<SessionLogger>,
    direction: Direction,
) -> io::Result<u64> {
    let mut total_bytes = 0_u64;
    let mut buf = [0_u8; 8192];

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            let _ = writer.shutdown(Shutdown::Write);
            break;
        }
        writer.write_all(&buf[..n])?;
        logger.log_chunk(direction, &buf[..n])?;
        total_bytes += n as u64;
    }

    Ok(total_bytes)
}

fn write_session_info(
    logger: &SessionLogger,
    listen: &str,
    target: &str,
    client_addr: &str,
    bytes_c2s: u64,
    bytes_s2c: u64,
) -> io::Result<()> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let mut file = File::create(logger.root().join("session.txt"))?;
    writeln!(file, "timestamp_unix={ts}")?;
    writeln!(file, "listen={listen}")?;
    writeln!(file, "target={target}")?;
    writeln!(file, "client={client_addr}")?;
    writeln!(file, "bytes_client_to_server={bytes_c2s}")?;
    writeln!(file, "bytes_server_to_client={bytes_s2c}")?;
    writeln!(
        file,
        "note=tcp_stream_has_no_request_boundaries_each_client_chunk_is_treated_as_new_request"
    )?;
    writeln!(
        file,
        "inflate_status=not_available_without_external_crates_in_current_offline_build"
    )?;
    Ok(())
}

fn parse_args() -> Result<Args, String> {
    let mut listen: Option<String> = None;
    let mut target: Option<String> = None;
    let mut log_dir = PathBuf::from("logs");
    let mut try_inflate = true;

    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--listen" => listen = it.next(),
            "--target" => target = it.next(),
            "--log-dir" => {
                if let Some(v) = it.next() {
                    log_dir = PathBuf::from(v);
                } else {
                    return Err("missing value for --log-dir".to_string());
                }
            }
            "--try-inflate" => {
                if let Some(v) = it.next() {
                    try_inflate = parse_bool(&v)?;
                } else {
                    return Err("missing value for --try-inflate".to_string());
                }
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            unknown => return Err(format!("unknown argument: {unknown}")),
        }
    }

    let listen = listen.ok_or_else(|| "missing required argument --listen".to_string())?;
    let target = target.ok_or_else(|| "missing required argument --target".to_string())?;
    Ok(Args {
        listen,
        target,
        log_dir,
        try_inflate,
    })
}

fn parse_bool(value: &str) -> Result<bool, String> {
    match value {
        "1" | "true" | "yes" | "on" => Ok(true),
        "0" | "false" | "no" | "off" => Ok(false),
        _ => Err(format!("invalid bool value: {value}")),
    }
}

fn print_help() {
    println!("v8-proxy");
    println!("Usage:");
    println!("  v8_protocols --listen <addr:port> --target <addr:port> [--log-dir <path>] [--try-inflate <true|false>]");
    println!();
    println!("Options:");
    println!("  --listen       Proxy listen address, example 127.0.0.1:15410");
    println!("  --target       Upstream server address, example 127.0.0.1:1541");
    println!("  --log-dir      Base log directory (default: logs)");
    println!("  --try-inflate  Reserved flag for future deflate decode support (default: true)");
}

fn run(args: Args) -> io::Result<()> {
    let listener = TcpListener::bind(&args.listen)?;
    println!("Listening on {}", args.listen);
    println!("Target server {}", args.target);

    let (client_stream, client_addr) = listener.accept()?;
    println!("Accepted client {}", client_addr);

    let server_stream = TcpStream::connect(&args.target)?;
    println!("Connected to target {}", args.target);

    let session_dir = session_directory(&args.log_dir, &client_addr.to_string());
    let logger = Arc::new(SessionLogger::new(session_dir)?);
    println!("Session log dir {}", logger.root().display());

    let c2s_reader = client_stream.try_clone()?;
    let c2s_writer = server_stream.try_clone()?;
    let s2c_reader = server_stream;
    let s2c_writer = client_stream;

    let logger_c2s = Arc::clone(&logger);
    let t1 = thread::spawn(move || {
        relay(
            c2s_reader,
            c2s_writer,
            logger_c2s,
            Direction::ClientToServer,
        )
    });

    let logger_s2c = Arc::clone(&logger);
    let t2 = thread::spawn(move || {
        relay(
            s2c_reader,
            s2c_writer,
            logger_s2c,
            Direction::ServerToClient,
        )
    });

    let bytes_c2s = t1
        .join()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "c2s thread panicked"))??;
    let bytes_s2c = t2
        .join()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "s2c thread panicked"))??;

    write_session_info(
        &logger,
        &args.listen,
        &args.target,
        &client_addr.to_string(),
        bytes_c2s,
        bytes_s2c,
    )?;
    generate_text_dumps(logger.root())?;

    if args.try_inflate {
        eprintln!(
            "Warning: --try-inflate is reserved in offline std-only build and is currently skipped"
        );
    }

    println!("Session complete");
    Ok(())
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Argument error: {err}");
            print_help();
            std::process::exit(2);
        }
    };

    if let Err(err) = run(args) {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

use clap::Parser;
use rac_cli::rac_lite::{run, Cli};
use rac_protocol::error::RacError;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("Error: {e}");
        if let RacError::Io(err) = &e {
            match err.kind() {
                std::io::ErrorKind::WouldBlock => {
                    eprintln!("Hint: temporary socket unavailability (EAGAIN). Retry the command.")
                }
                std::io::ErrorKind::TimedOut => {
                    eprintln!("Hint: operation timed out. Check server availability and network.")
                }
                std::io::ErrorKind::InvalidInput => {
                    eprintln!("Hint: invalid address or port. Check for stray spaces/CRLF.")
                }
                _ => {}
            }
        }
        std::process::exit(1);
    }
}

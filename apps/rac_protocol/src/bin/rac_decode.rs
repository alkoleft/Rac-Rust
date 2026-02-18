use std::env;
use std::fs;
use std::path::Path;

use rac_protocol::rac_wire::{decode_rpc_method, parse_frames, Frame};

fn hex(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len() * 2);
    for b in data {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

fn usage(bin: &str) {
    eprintln!("Usage: {bin} <stream.bin>");
}

fn main() {
    let mut args = env::args();
    let bin = args.next().unwrap_or_else(|| "rac_decode".to_string());
    let Some(path) = args.next() else {
        usage(&bin);
        std::process::exit(2);
    };
    if args.next().is_some() {
        usage(&bin);
        std::process::exit(2);
    }

    let file = Path::new(&path);
    let data = match fs::read(file) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("read error: {e}");
            std::process::exit(1);
        }
    };

    println!("file={}", file.display());
    println!("size={}", data.len());
    let frames = match parse_frames(&data) {
        Ok(frames) => frames,
        Err(err) => {
            eprintln!("frame parse error: {err}");
            std::process::exit(1);
        }
    };
    println!("frames={}", frames.len());

    for (idx, frame) in frames.iter().enumerate() {
        dump_frame(idx + 1, frame);
    }
}

fn dump_frame(idx: usize, frame: &Frame) {
    println!(
        "frame={} opcode=0x{:02x} len={} len_field_size={}",
        idx,
        frame.opcode,
        frame.payload.len(),
        frame.len_field_size
    );
    println!("  payload_hex={}", hex(&frame.payload));
    if let Some(method) = decode_rpc_method(&frame.payload) {
        println!("  rpc_method_id=0x{method:x} ({method})");
    }
}

use crate::rac_wire::Frame;

pub(crate) fn format_payload_head(payload: &[u8], max_len: usize) -> String {
    let take = payload.len().min(max_len);
    if take == 0 {
        return "<empty>".to_string();
    }
    let mut out = String::new();
    for (idx, byte) in payload[..take].iter().enumerate() {
        if idx > 0 {
            out.push(' ');
        }
        out.push_str(&format!("{byte:02x}"));
    }
    if payload.len() > take {
        out.push_str(" ...");
    }
    out
}

pub(crate) fn payload_has_service_name(payload: &[u8]) -> bool {
    payload
        .windows(b"v8.service.Admin.Cluster".len())
        .any(|w| w == b"v8.service.Admin.Cluster")
}

pub(crate) fn log_frame(label: &str, frame: &Frame) {
    let mut hex = String::new();
    for (idx, b) in frame.payload.iter().enumerate() {
        if idx > 0 {
            hex.push(' ');
        }
        hex.push_str(&format!("{b:02x}"));
    }
    eprintln!(
        "rac_lite debug: {label}: opcode=0x{:02x} len={} payload_hex={}",
        frame.opcode,
        frame.payload.len(),
        hex
    );
}

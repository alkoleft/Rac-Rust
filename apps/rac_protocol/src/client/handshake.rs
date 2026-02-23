use crate::client::debug::log_frame;
use crate::client::transport::RacTransport;
use crate::error::{RacError, Result};
use crate::protocol::ProtocolCodec;

pub(crate) fn negotiate(
    transport: &mut RacTransport,
    protocol: &dyn ProtocolCodec,
    debug_raw: bool,
) -> Result<()> {
    transport.write_raw(protocol.init_packet())?;
    transport.flush()?;

    let ack = transport.read_frame()?;
    if ack.opcode != protocol.opcode_init_ack() {
        if debug_raw {
            log_frame("init-ack-unexpected", &ack);
        }
        return Err(RacError::ProtocolMessage(format!(
            "unexpected init ack opcode 0x{:02x}",
            ack.opcode
        )));
    }

    transport.write_frame(
        protocol.opcode_service_negotiation(),
        protocol.service_negotiation_payload(),
    )?;
    transport.flush()?;

    for _ in 0..3 {
        let svc = transport.read_frame()?;
        if svc.opcode == protocol.opcode_service_ack() {
            return Ok(());
        }
        if svc.opcode == 0x0f {
            if is_unsupported_service(&svc.payload) {
                if debug_raw {
                    log_frame("service-unsupported", &svc);
                }
                return Err(RacError::UnsupportedService {
                    payload: svc.payload,
                });
            }
            if debug_raw {
                log_frame("service-notice-unexpected", &svc);
            }
            continue;
        }
        if debug_raw {
            log_frame("service-ack-unexpected", &svc);
        }
        return Err(RacError::Protocol("unexpected service negotiation reply"));
    }

    Err(RacError::Protocol(
        "service negotiation reply not received",
    ))
}

fn is_unsupported_service(payload: &[u8]) -> bool {
    const MARKER: &[u8] = b"UnsupportedService";
    payload
        .windows(MARKER.len())
        .any(|window| window == MARKER)
}

use crate::client::debug::log_frame;
use crate::client::protocol::RacProtocol;
use crate::client::transport::RacTransport;
use crate::error::{RacError, Result};

pub(crate) fn negotiate(
    transport: &mut RacTransport,
    protocol: &dyn RacProtocol,
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

    let svc = transport.read_frame()?;
    if svc.opcode != protocol.opcode_service_ack() {
        if debug_raw {
            log_frame("service-ack-unexpected", &svc);
        }
        return Err(RacError::Protocol("unexpected service negotiation reply"));
    }

    Ok(())
}

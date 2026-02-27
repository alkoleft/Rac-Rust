use crate::client::RacClient;
use crate::error::Result;
use crate::Uuid16;

mod generated {
    include!("profile_generated.rs");
}

pub use generated::{ProfileListResp, ProfileListRpc, ProfileRecord, ProfileUpdateRpc};

use crate::rpc::AckResponse;

pub fn profile_list(client: &mut RacClient, cluster: Uuid16) -> Result<ProfileListResp> {
    client.call_typed(ProfileListRpc { cluster })
}

pub fn profile_update(client: &mut RacClient, req: ProfileUpdateRpc) -> Result<AckResponse> {
    client.call_typed(req)
}

#[cfg(all(test, feature = "artifacts"))]
mod tests {
    use super::*;
    use crate::protocol::ProtocolVersion;
    use crate::rac_wire::parse_uuid;
    use crate::rpc::Request;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn encode_profile_update_request_all_yes() {
        let expected = decode_hex_str(
            include_str!("../../../../artifacts/rac/v11/v11_profile_update_all_yes_request.hex"),
        );
        let cluster =
            parse_uuid("95a0a524-eeae-43f7-a659-627211c32d5e").expect("cluster uuid");
        let rpc = ProfileUpdateRpc {
            cluster,
            name: "codex_prof_all_yes".to_string(),
            descr: "codex_all_yes".to_string(),
            directory_access: 0,
            com_access: 0,
            addin_access: 0,
            module_access: 0,
            app_access: 0,
            config: 1,
            privileged_mode: 1,
            inet_access: 0,
            crypto: 1,
            right_extension: 1,
            right_extension_definition_roles: "role3;role4".to_string(),
            all_modules_extension: 1,
            modules_available_for_extension: "mod1;mod2".to_string(),
            modules_not_available_for_extension: "mod3;mod4".to_string(),
            privileged_mode_roles: "role1;role2".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = rpc.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }
}

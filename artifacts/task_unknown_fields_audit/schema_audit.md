# Schema audit (unknown/gap/tail markers)

## agent.toml
- markers: none

## cluster.toml
- markers:
  - 6:  { name = "unknown_flags", type = "u32_be", version = "11.0" },
  - 115:  { index = 0, field = "unknown_flags", value = 0x03efbfbd },
  - 130:body = { type = "list_u8_tail", item = "ClusterRecord", tail_len_param = "tail_len" }
  - 132:  { name = "cluster_list_response_custom_hex", hex_path = "../../../../artifacts/rac/cluster_list_response_custom.hex", tail_len = 0, expect_len = 1, asserts = [
  - 141:  { name = "cluster_list_response_flags_hex", hex_path = "../../../../artifacts/rac/cluster_list_response_flags.hex", tail_len = 0, expect_len = 1, asserts = [
  - 148:body = { type = "record_tail", item = "ClusterRecord", tail_len_param = "tail_len" }

## connection.toml
- markers: none

## console_output.toml
- markers:
  - 223:  { label = "unknown-flags", value = "unknown_flags", format = "hex_u32" },

## counter.toml
- markers: none

## infobase.toml
- markers:
  - 14:  { name = "unknown_u32_0", type = "u32_be", version = "11.0" },
  - 17:  { name = "unknown_str_0", type = "str8", version = "11.0" },
  - 20:  { name = "unknown_str_1", type = "str8", version = "11.0" },
  - 21:  { name = "unknown_str_2", type = "str8", version = "11.0" },
  - 22:  { name = "unknown_bytes_0", type = "bytes_fixed", len = 4, version = "11.0" },
  - 25:  { name = "unknown_str_3", type = "str8", version = "11.0" },
  - 26:  { name = "unknown_str_4", type = "str8", version = "11.0" },
  - 27:  { name = "unknown_u32_1", type = "u32_be", version = "11.0" },
  - 32:  { name = "tail", type = "bytes_fixed", len = 28, version = "11.0" },

## limit.toml
- markers: none

## lock.toml
- markers: none

## manager.toml
- markers: none

## process.toml
- markers:
  - 4:  { name = "_gap_license_0", type = "u8", skip = true, version = "11.0" },
  - 23:  { name = "_gap_0", type = "bytes", len = 8, skip = true, version = "11.0" },

## profile.toml
- markers: none

## rule.toml
- markers: none

## server.toml
- markers:
  - 10:  { name = "gap_1", type = "u32_le", version = "11.0" },
  - 12:  { name = "gap_2", type = "u32_le", version = "11.0" },
  - 14:  { name = "gap_3", type = "u32_le", version = "11.0" },
  - 15:  { name = "gap_4", type = "u32_le", version = "11.0" },
  - 16:  { name = "gap_4_pad", type = "u8", version = "11.0" },
  - 22:  { name = "gap_5", type = "u32_be", version = "11.0" },
  - 24:  { name = "gap_6", type = "u32_be", version = "11.0" },
  - 28:  { name = "gap_7", type = "u8", version = "11.0" },

## service_setting.toml
- markers: none

## session.toml
- markers: none

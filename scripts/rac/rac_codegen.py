#!/usr/bin/env python3
import argparse
try:
    import tomllib  # type: ignore
    _TOML_AVAILABLE = True
except ModuleNotFoundError:  # Python < 3.11
    _TOML_AVAILABLE = False
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple


ROOT = Path(__file__).resolve().parents[1]


@dataclass
class FieldSpec:
    name: str
    type_name: str
    item: Optional[str] = None
    length: Optional[int] = None
    len_source: Optional[str] = None
    skip: bool = False
    computed: Optional[str] = None
    source: Optional[str] = None
    rust_type: Optional[str] = None
    literal: Optional[List[int]] = None


@dataclass
class RecordSpec:
    name: str
    derives: List[str]
    fields: List[FieldSpec]


@dataclass
class RequestSpec:
    name: str
    derives: List[str]
    fields: List[FieldSpec]


@dataclass
class RpcTestSpec:
    name: str
    hex_path: str
    args: Dict[str, Any]
    protocol: str


@dataclass
class RpcSpec:
    name: str
    request: Optional[str]
    method_req: int
    method_resp: Optional[int]
    requires_cluster_context: bool
    requires_infobase_context: bool
    tests: List[RpcTestSpec]


@dataclass
class ResponseAssertSpec:
    field: str
    value: Any
    index: Optional[int] = None


@dataclass
class ResponseTestSpec:
    name: str
    hex_path: str
    expect_len: Optional[int]
    asserts: List[ResponseAssertSpec]


@dataclass
class ResponseBodySpec:
    type_name: str
    item: Optional[str]


@dataclass
class ResponseSpec:
    name: str
    body: ResponseBodySpec
    tests: List[ResponseTestSpec]


def parse_schema(
    path: Path,
) -> Tuple[List[RecordSpec], List[RequestSpec], List[RpcSpec], List[ResponseSpec]]:
    if _TOML_AVAILABLE:
        payload = tomllib.loads(path.read_text())
        return parse_schema_payload(payload)
    return parse_schema_minimal(path)


def parse_schema_payload(
    payload: Dict[str, Any],
) -> Tuple[List[RecordSpec], List[RequestSpec], List[RpcSpec], List[ResponseSpec]]:
    records: List[RecordSpec] = []
    requests: List[RequestSpec] = []
    rpcs: List[RpcSpec] = []
    responses: List[ResponseSpec] = []
    record_table: Dict[str, Any] = payload.get("record", {})
    for name, spec in record_table.items():
        derives = [str(v) for v in spec.get("derive", ["Debug", "Serialize", "Clone"])]
        fields = []
        for raw in spec.get("fields", []):
            fields.append(
                FieldSpec(
                    name=str(raw.get("name", "")),
                    type_name=str(raw.get("type", "")),
                    item=raw.get("item"),
                    length=raw.get("len"),
                    len_source=raw.get("len_source"),
                    skip=bool(raw.get("skip", False)),
                    computed=raw.get("computed"),
                    source=raw.get("source"),
                    rust_type=raw.get("rust_type"),
                    literal=raw.get("literal"),
                )
            )
        records.append(RecordSpec(name=name, derives=derives, fields=fields))
    request_table: Dict[str, Any] = payload.get("request", {})
    for name, spec in request_table.items():
        derives = [str(v) for v in spec.get("derive", ["Debug", "Clone"])]
        fields = []
        for raw in spec.get("fields", []):
            fields.append(
                FieldSpec(
                    name=str(raw.get("name", "")),
                    type_name=str(raw.get("type", "")),
                    item=raw.get("item"),
                    length=raw.get("len"),
                    len_source=raw.get("len_source"),
                    skip=bool(raw.get("skip", False)),
                    computed=raw.get("computed"),
                    source=raw.get("source"),
                    rust_type=raw.get("rust_type"),
                    literal=raw.get("literal"),
                )
            )
        requests.append(RequestSpec(name=name, derives=derives, fields=fields))
    rpc_table: Dict[str, Any] = payload.get("rpc", {})
    for name, spec in rpc_table.items():
        tests = []
        for raw in spec.get("tests", []):
            tests.append(
                RpcTestSpec(
                    name=str(raw.get("name", f"{name}_request_hex")),
                    hex_path=str(raw.get("hex_path", "")),
                    args=raw.get("args", {}) or {},
                    protocol=str(raw.get("protocol", "v16.0")),
                )
            )
        rpcs.append(
            RpcSpec(
                name=name,
                request=spec.get("request"),
                method_req=int(spec.get("method_req")),
                method_resp=spec.get("method_resp"),
                requires_cluster_context=bool(spec.get("requires_cluster_context", False)),
                requires_infobase_context=bool(spec.get("requires_infobase_context", False)),
                tests=tests,
            )
        )
    response_table: Dict[str, Any] = payload.get("response", {})
    for name, spec in response_table.items():
        body_spec = spec.get("body") or {}
        body = ResponseBodySpec(
            type_name=str(body_spec.get("type", "")),
            item=body_spec.get("item"),
        )
        tests = []
        for raw in spec.get("tests", []):
            asserts = []
            for assert_raw in raw.get("asserts", []) or []:
                asserts.append(
                    ResponseAssertSpec(
                        field=str(assert_raw.get("field", "")),
                        value=assert_raw.get("value"),
                        index=assert_raw.get("index"),
                    )
                )
            tests.append(
                ResponseTestSpec(
                    name=str(raw.get("name", f"{name}_response_hex")),
                    hex_path=str(raw.get("hex_path", "")),
                    expect_len=raw.get("expect_len"),
                    asserts=asserts,
                )
            )
        responses.append(ResponseSpec(name=name, body=body, tests=tests))
    return records, requests, rpcs, responses


def parse_schema_minimal(
    path: Path,
) -> Tuple[List[RecordSpec], List[RequestSpec], List[RpcSpec], List[ResponseSpec]]:
    lines = path.read_text().splitlines()
    records: Dict[str, Dict[str, Any]] = {}
    requests: Dict[str, Dict[str, Any]] = {}
    rpcs: Dict[str, Dict[str, Any]] = {}
    responses: Dict[str, Dict[str, Any]] = {}
    current: Optional[Dict[str, Any]] = None
    current_name: Optional[str] = None
    current_kind: Optional[str] = None
    in_fields = False
    fields_buf: List[Dict[str, Any]] = []

    for raw in lines:
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        if (
            line.startswith("[record.")
            and line.endswith("]")
            or line.startswith("[request.")
            and line.endswith("]")
            or line.startswith("[rpc.")
            and line.endswith("]")
            or line.startswith("[response.")
            and line.endswith("]")
        ):
            if current_name and current is not None and current_kind:
                current["fields"] = fields_buf
                if current_kind == "record":
                    records[current_name] = current
                elif current_kind == "request":
                    requests[current_name] = current
                elif current_kind == "rpc":
                    rpcs[current_name] = current
                else:
                    responses[current_name] = current
            if line.startswith("[record."):
                current_kind = "record"
                current_name = line[len("[record.") : -1]
            elif line.startswith("[request."):
                current_kind = "request"
                current_name = line[len("[request.") : -1]
            elif line.startswith("[rpc."):
                current_kind = "rpc"
                current_name = line[len("[rpc.") : -1]
            else:
                current_kind = "response"
                current_name = line[len("[response.") : -1]
            current = {}
            fields_buf = []
            in_fields = False
            continue
        if current is None:
            continue
        if line.startswith("derive"):
            current["derive"] = parse_list_value(line.split("=", 1)[1].strip())
            continue
        if line.startswith("fields"):
            in_fields = True
            if line.endswith("]"):
                in_fields = False
            continue
        if "=" in line and not in_fields:
            key, raw_val = line.split("=", 1)
            current[key.strip()] = parse_value(raw_val.strip())
            continue
        if in_fields:
            if line.startswith("]"):
                in_fields = False
                continue
            if line.endswith(","):
                line = line[:-1].strip()
            if line.startswith("{") and line.endswith("}"):
                fields_buf.append(parse_inline_table(line))
            continue

    if current_name and current is not None and current_kind:
        current["fields"] = fields_buf
        if current_kind == "record":
            records[current_name] = current
        elif current_kind == "request":
            requests[current_name] = current
        elif current_kind == "rpc":
            rpcs[current_name] = current
        else:
            responses[current_name] = current

    return parse_schema_payload({"record": records, "request": requests, "rpc": rpcs, "response": responses})


def parse_list_value(value: str) -> List[str]:
    value = value.strip()
    if not value.startswith("[") or not value.endswith("]"):
        return []
    inner = value[1:-1].strip()
    if not inner:
        return []
    parts = split_top_level(inner, ",")
    return [strip_quotes(p.strip()) for p in parts if p.strip()]


def parse_inline_table(value: str) -> Dict[str, Any]:
    inner = value.strip()[1:-1].strip()
    items = split_top_level(inner, ",")
    out: Dict[str, Any] = {}
    for item in items:
        if "=" not in item:
            continue
        key, raw_val = item.split("=", 1)
        key = key.strip()
        raw_val = raw_val.strip()
        out[key] = parse_value(raw_val)
    return out


def parse_value(raw_val: str) -> Any:
    if raw_val.startswith("\"") and raw_val.endswith("\""):
        return strip_quotes(raw_val)
    if raw_val.startswith("{") and raw_val.endswith("}"):
        return parse_inline_table(raw_val)
    if raw_val.startswith("[") and raw_val.endswith("]"):
        inner = raw_val[1:-1].strip()
        if not inner:
            return []
        parts = split_top_level(inner, ",")
        return [parse_value(p.strip()) for p in parts if p.strip()]
    if raw_val in {"true", "false"}:
        return raw_val == "true"
    if raw_val.startswith("0x") and all(ch in "0123456789abcdefABCDEF" for ch in raw_val[2:]):
        return int(raw_val, 16)
    if raw_val.isdigit():
        return int(raw_val)
    return strip_quotes(raw_val)


def strip_quotes(value: str) -> str:
    if value.startswith("\"") and value.endswith("\""):
        return value[1:-1]
    return value


def split_top_level(value: str, sep: str) -> List[str]:
    parts: List[str] = []
    buf: List[str] = []
    in_str = False
    escaped = False
    depth = 0
    for ch in value:
        if escaped:
            buf.append(ch)
            escaped = False
            continue
        if ch == "\\" and in_str:
            escaped = True
            buf.append(ch)
            continue
        if ch == "\"":
            in_str = not in_str
            buf.append(ch)
            continue
        if ch in "[{":
            depth += 1
            buf.append(ch)
            continue
        if ch in "]}":
            if depth > 0:
                depth -= 1
            buf.append(ch)
            continue
        if ch == sep and not in_str and depth == 0:
            parts.append("".join(buf))
            buf = []
            continue
        buf.append(ch)
    if buf:
        parts.append("".join(buf))
    return parts


def rust_type(field: FieldSpec) -> str:
    if field.rust_type:
        return field.rust_type
    if field.type_name == "uuid":
        return "Uuid16"
    if field.type_name in {
        "str8",
        "str8_opt",
        "str_len_u8",
        "str_len_u8_or_2c",
        "str_u14",
        "datetime_u64_be",
        "datetime_u64_be_opt",
    }:
        return "String"
    if field.type_name == "bytes":
        return "Vec<u8>"
    if field.type_name == "bytes_fixed":
        if field.length is None:
            raise ValueError("bytes_fixed requires len")
        return f"[u8; {field.length}]"
    if field.type_name == "lock_descr":
        return "LockDescr"
    if field.type_name == "u8":
        return "u8"
    if field.type_name == "u16_be":
        return "u16"
    if field.type_name == "u16_le":
        return "u16"
    if field.type_name == "u16_be_bool":
        return "bool"
    if field.type_name == "u32_be":
        return "u32"
    if field.type_name == "u32_le":
        return "u32"
    if field.type_name == "u32_be_opt":
        return "u32"
    if field.type_name == "u64_be":
        return "u64"
    if field.type_name == "u64_be_opt":
        return "u64"
    if field.type_name == "f64_be":
        return "f64"
    if field.type_name in {"u8_bool", "u32_be_bool"}:
        return "bool"
    if field.type_name == "u8_opt":
        return "u8"
    if field.type_name == "bool_opt":
        return "bool"
    if field.type_name == "uuid_opt":
        return "Uuid16"
    if field.type_name == "list_u8":
        if not field.item:
            raise ValueError("list_u8 requires item")
        return f"Vec<{field.item}>"
    if field.type_name == "list_str8_rest":
        return "Vec<String>"
    if field.type_name == "record":
        if not field.item:
            raise ValueError("record requires item")
        return field.item
    if field.type_name == "record_u8_first":
        if not field.item:
            raise ValueError("record_u8_first requires item")
        return field.item
    if field.type_name == "computed":
        return field.rust_type or "bool"
    raise ValueError(f"unknown type: {field.type_name}")


def decode_expr(field: FieldSpec, var_map: Dict[str, str]) -> List[str]:
    t = field.type_name
    if t == "uuid":
        return ["cursor.take_uuid()?;"]
    if t == "uuid_opt":
        return ["cursor.take_uuid_opt()?.unwrap_or_default();"]
    if t == "str8":
        return ["cursor.take_str8()?;"]
    if t == "str8_opt":
        return ["cursor.take_str8_opt()?.unwrap_or_default();"]
    if t == "str_len_u8":
        if field.len_source:
            len_var = var_map.get(field.len_source, field.len_source)
            return [
                f"let len = {len_var} as usize;",
                "let bytes = cursor.take_bytes(len)?;",
                "String::from_utf8_lossy(&bytes).to_string()",
            ]
        return [
            "let len = cursor.take_u8()? as usize;",
            "let bytes = cursor.take_bytes(len)?;",
            "String::from_utf8_lossy(&bytes).to_string()",
        ]
    if t == "str_len_u8_or_2c":
        return [
            "let first = cursor.take_u8()? as usize;",
            "let len = if first == 0x2c { cursor.take_u8()? as usize } else { first };",
            "let bytes = cursor.take_bytes(len)?;",
            "String::from_utf8_lossy(&bytes).to_string()",
        ]
    if t == "str_u14":
        return [
            "let b0 = cursor.take_u8()? as usize;",
            "let b1 = cursor.take_u8()? as usize;",
            "let len = (b0 & 0x3f) | (b1 << 6);",
            "let bytes = cursor.take_bytes(len)?;",
            "String::from_utf8_lossy(&bytes).to_string()",
        ]
    if t == "bytes":
        if field.length is None:
            raise ValueError("bytes requires len")
        return [f"cursor.take_bytes({field.length})?;"]
    if t == "bytes_fixed":
        if field.length is None:
            raise ValueError("bytes_fixed requires len")
        arr_len = field.length
        return [
            f"let bytes = cursor.take_bytes({arr_len})?;",
            f"let value: [u8; {arr_len}] = bytes.as_slice().try_into()"
            f".map_err(|_| RacError::Decode(\"bytes_fixed\"))?;",
            "value",
        ]
    if t == "lock_descr":
        return [
            "let descr_len = cursor.take_u8()? as usize;",
            "if descr_len == 0 {",
            "    LockDescr { descr: String::new(), descr_flag: None }",
            "} else {",
            "    let first = cursor.take_u8()?;",
            "    let remaining = cursor.remaining_len();",
            "    let needed_no_flag = descr_len.saturating_sub(1) + 40;",
            "    let needed_flag = descr_len + 40;",
            "    let use_flag = if first == 0x01 {",
            "        if remaining == needed_flag {",
            "            true",
            "        } else if remaining == needed_no_flag {",
            "            false",
            "        } else if remaining >= needed_flag && remaining < needed_no_flag {",
            "            true",
            "        } else if remaining >= needed_no_flag {",
            "            false",
            "        } else {",
            "            remaining >= needed_flag",
            "        }",
            "    } else {",
            "        false",
            "    };",
            "    if use_flag {",
            "        let descr_bytes = cursor.take_bytes(descr_len)?;",
            "        let descr = String::from_utf8(descr_bytes)",
            "            .map_err(|_| RacError::Decode(\"lock descr invalid utf-8\"))?;",
            "        LockDescr { descr, descr_flag: Some(first) }",
            "    } else {",
            "        let mut descr_bytes = Vec::with_capacity(descr_len);",
            "        descr_bytes.push(first);",
            "        if descr_len > 1 {",
            "            descr_bytes.extend_from_slice(&cursor.take_bytes(descr_len - 1)?);",
            "        }",
            "        let descr = String::from_utf8(descr_bytes)",
            "            .map_err(|_| RacError::Decode(\"lock descr invalid utf-8\"))?;",
            "        LockDescr { descr, descr_flag: None }",
            "    }",
            "}",
        ]
    if t == "u8":
        return ["cursor.take_u8()?;"]
    if t == "u8_opt":
        return ["cursor.take_u8_opt()?.unwrap_or_default();"]
    if t == "u16_be":
        return ["cursor.take_u16_be()?;"]
    if t == "u16_le":
        return ["cursor.take_u16_le()?;"]
    if t == "u32_be":
        return ["cursor.take_u32_be()?;"]
    if t == "u32_le":
        return ["cursor.take_u32_le()?;"]
    if t == "u32_be_opt":
        return ["cursor.take_u32_be_opt()?.unwrap_or_default();"]
    if t == "u64_be":
        return ["cursor.take_u64_be()?;"]
    if t == "u64_be_opt":
        return ["cursor.take_u64_be_opt()?.unwrap_or_default();"]
    if t == "f64_be":
        return ["cursor.take_f64_be()?;"]
    if t == "u8_bool":
        return ["cursor.take_u8()? != 0;"]
    if t == "u32_be_bool":
        return ["cursor.take_u32_be()? != 0;"]
    if t == "u16_be_bool":
        return ["cursor.take_u16_be()? != 0;"]
    if t == "bool_opt":
        return ["cursor.take_bool_opt()?.unwrap_or_default();"]
    if t == "datetime_u64_be":
        return ["v8_datetime_to_iso(cursor.take_u64_be()?).unwrap_or_default();"]
    if t == "datetime_u64_be_opt":
        return ["cursor.take_datetime_opt()?.unwrap_or_default();"]
    if t == "list_u8":
        item = field.item
        if not item:
            raise ValueError("list_u8 requires item")
        return [
            "let count = cursor.take_u8()? as usize;",
            "let mut out = Vec::with_capacity(count);",
            "for _ in 0..count {",
            f"    out.push({item}::decode(cursor)?);",
            "}",
            "out",
        ]
    if t == "list_str8_rest":
        return [
            "let mut out = Vec::new();",
            "while cursor.remaining_len() > 0 {",
            "    out.push(cursor.take_str8()?);",
            "}",
            "out",
        ]
    if t == "record":
        item = field.item
        if not item:
            raise ValueError("record requires item")
        return [f"{item}::decode(cursor)?;"]
    if t == "record_u8_first":
        item = field.item
        if not item:
            raise ValueError("record_u8_first requires item")
        return [
            "let count = cursor.take_u8()? as usize;",
            f"if count == 0 {{ {item}::default() }} else {{ {item}::decode(cursor)? }}",
        ]
    raise ValueError(f"unknown type for decode: {t}")


def needs_datetime(records: List[RecordSpec]) -> bool:
    for record in records:
        for field in record.fields:
            if field.type_name == "datetime_u64_be":
                return True
    return False


def needs_uuid(records: List[RecordSpec]) -> bool:
    for record in records:
        for field in record.fields:
            if field.type_name in {"uuid", "uuid_opt"}:
                return True
    return False


def needs_rac_error(records: List[RecordSpec]) -> bool:
    for record in records:
        for field in record.fields:
            if field.type_name in {"bytes_fixed", "lock_descr"}:
                return True
    return False


def needs_protocol_version(records: List[RecordSpec]) -> bool:
    for record in records:
        for field in record.fields:
            if field.rust_type == "RacProtocolVersion":
                return True
            if field.source and "RacProtocolVersion" in field.source:
                return True
    return False


def needs_rac_error_responses(responses: List[ResponseSpec]) -> bool:
    for resp in responses:
        if resp.body.type_name == "record":
            return True
    return False


def generate(
    records: List[RecordSpec],
    responses: List[ResponseSpec],
    rpcs: List[RpcSpec],
    extra_uses: Optional[List[str]] = None,
) -> str:
    lines: List[str] = []
    uses = ["use crate::codec::RecordCursor;", "use crate::error::Result;"]
    if needs_datetime(records):
        uses.insert(0, "use crate::codec::v8_datetime_to_iso;")
    if needs_rac_error(records) or needs_rac_error_responses(responses):
        uses.insert(0, "use crate::error::RacError;")
    if needs_uuid(records):
        uses.insert(0, "use crate::Uuid16;")
    if needs_protocol_version(records):
        uses.insert(0, "use crate::client::RacProtocolVersion;")
    uses.append("use serde::Serialize;")
    if extra_uses:
        for item in extra_uses:
            if item not in uses:
                uses.append(item)
    lines.extend(uses)
    lines.append("")

    for record in records:
        derive = ", ".join(record.derives)
        lines.append(f"#[derive({derive})]")
        lines.append(f"pub struct {record.name} {{")
        for field in record.fields:
            if field.skip:
                continue
            if field.computed:
                rust_ty = rust_type(field)
                lines.append(f"    pub {field.name}: {rust_ty},")
                continue
            rust_ty = rust_type(field)
            lines.append(f"    pub {field.name}: {rust_ty},")
        lines.append("}")
        lines.append("")
        lines.append(f"impl {record.name} {{")
        lines.append("    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {")

        computed_lines: List[str] = []
        var_map: Dict[str, str] = {}
        for field in record.fields:
            if field.computed:
                if not field.source:
                    raise ValueError("computed fields require source")
                if field.computed == "ne_zero":
                    computed_lines.append(f"        let {field.name} = {field.source} != 0;")
                elif field.computed == "literal":
                    computed_lines.append(f"        let {field.name} = {field.source};")
                else:
                    raise ValueError("computed fields must use computed='ne_zero' or computed='literal'")
                continue
            var_name = field.name
            if field.skip:
                var_name = f"_{field.name}"
            var_map[field.name] = var_name
            expr = decode_expr(field, var_map)
            if len(expr) == 1:
                lines.append(f"        let {var_name} = {expr[0]}")
            else:
                lines.append(f"        let {var_name} = {{")
                for step in expr[:-1]:
                    lines.append(f"            {step}")
                lines.append(f"            {expr[-1]}")
                lines.append("        };")

        if computed_lines:
            lines.append("")
            lines.extend(computed_lines)

        lines.append("        Ok(Self {")
        for field in record.fields:
            if field.skip:
                continue
            lines.append(f"            {field.name},")
        lines.append("        })")
        lines.append("    }")
        lines.append("}")
        lines.append("")

    if responses:
        lines.extend(generate_response_parsers(responses))
        lines.append("")

    if rpcs:
        lines.extend(generate_rpc_metadata(rpcs))
        lines.append("")

    return "\n".join(lines).rstrip() + "\n"


def request_rust_type(field: FieldSpec) -> str:
    return rust_type(field)


def request_literal_bytes(field: FieldSpec) -> List[int]:
    if field.literal is None:
        return []
    if not isinstance(field.literal, list):
        raise ValueError("literal must be a list")
    out: List[int] = []
    for val in field.literal:
        if not isinstance(val, int):
            raise ValueError("literal list must contain integers")
        if val < 0 or val > 255:
            raise ValueError("literal byte out of range")
        out.append(val)
    return out


def request_encoded_len(field: FieldSpec) -> int:
    if field.literal is not None:
        if field.type_name == "bytes_fixed":
            return len(request_literal_bytes(field))
        if field.type_name == "u8":
            return 1
        raise ValueError("literal supported only for bytes_fixed and u8")
    t = field.type_name
    if t == "uuid":
        return 16
    if t == "u8":
        return 1
    if t == "u16_be":
        return 2
    if t == "u32_be":
        return 4
    if t == "u64_be":
        return 8
    if t == "bytes_fixed":
        if field.length is None:
            raise ValueError("bytes_fixed requires len")
        return field.length
    raise ValueError(f"unknown type for encoded_len: {t}")


def request_needs_uuid(requests: List[RequestSpec]) -> bool:
    for req in requests:
        for field in req.fields:
            if field.type_name == "uuid":
                return True
    return False


def request_needs_serde(requests: List[RequestSpec]) -> bool:
    for req in requests:
        if any(derive == "Serialize" for derive in req.derives):
            return True
    return False


def request_needs_encode_with_len_u8(requests: List[RequestSpec]) -> bool:
    for req in requests:
        for field in req.fields:
            if field.type_name == "str8":
                return True
    return False


def request_encode_expr(field: FieldSpec) -> List[str]:
    t = field.type_name
    if field.literal is not None:
        literal_bytes = request_literal_bytes(field)
        if t == "bytes_fixed":
            return [f"out.extend_from_slice(&{literal_bytes});"]
        if t == "u8":
            if len(literal_bytes) != 1:
                raise ValueError("u8 literal must have length 1")
            return [f"out.push({literal_bytes[0]});"]
        raise ValueError("literal supported only for bytes_fixed and u8")
    if t == "uuid":
        return [f"out.extend_from_slice(&self.{field.name});"]
    if t == "str8":
        return [f"out.extend_from_slice(&encode_with_len_u8(self.{field.name}.as_bytes())?);"]
    if t == "u8":
        return [f"out.push(self.{field.name});"]
    if t == "u16_be":
        return [f"out.extend_from_slice(&self.{field.name}.to_be_bytes());"]
    if t == "u32_be":
        return [f"out.extend_from_slice(&self.{field.name}.to_be_bytes());"]
    if t == "u64_be":
        return [f"out.extend_from_slice(&self.{field.name}.to_be_bytes());"]
    if t == "bytes_fixed":
        return [f"out.extend_from_slice(&self.{field.name});"]
    raise ValueError(f"unknown type for encode: {t}")


def request_uses(requests: List[RequestSpec]) -> List[str]:
    uses = ["use crate::error::Result;"]
    if request_needs_encode_with_len_u8(requests):
        uses.insert(0, "use crate::rac_wire::encode_with_len_u8;")
    if request_needs_uuid(requests):
        uses.insert(0, "use crate::Uuid16;")
    if request_needs_serde(requests):
        uses.append("use serde::Serialize;")
    return uses


def rpc_tests_use_protocol(rpcs: List[RpcSpec]) -> bool:
    return any(rpc.tests for rpc in rpcs)


def generate_rpc_metadata(rpcs: List[RpcSpec]) -> List[str]:
    lines: List[str] = []
    lines.append("#[derive(Debug, Clone, Copy)]")
    lines.append("pub struct RpcMethodMeta {")
    lines.append("    pub method_req: u8,")
    lines.append("    pub method_resp: Option<u8>,")
    lines.append("    pub requires_cluster_context: bool,")
    lines.append("    pub requires_infobase_context: bool,")
    lines.append("}")
    lines.append("")
    for rpc in rpcs:
        const_name = f"RPC_{snake_case(rpc.name).upper()}_META"
        method_resp = "None" if rpc.method_resp is None else f"Some({rpc.method_resp})"
        lines.append(f"pub const {const_name}: RpcMethodMeta = RpcMethodMeta {{")
        lines.append(f"    method_req: {rpc.method_req},")
        lines.append(f"    method_resp: {method_resp},")
        lines.append(f"    requires_cluster_context: {str(rpc.requires_cluster_context).lower()},")
        lines.append(f"    requires_infobase_context: {str(rpc.requires_infobase_context).lower()},")
        lines.append("};")
        lines.append("")

    lines.append("#[allow(dead_code)]")
    lines.append("pub fn rpc_metadata(request: &crate::client::RacRequest) -> Option<RpcMethodMeta> {")
    lines.append("    match request {")
    for rpc in rpcs:
        const_name = f"RPC_{snake_case(rpc.name).upper()}_META"
        if rpc.request is None:
            lines.append(f"        crate::client::RacRequest::{rpc.name} => Some({const_name}),")
        else:
            lines.append(f"        crate::client::RacRequest::{rpc.name} {{ .. }} => Some({const_name}),")
    lines.append("        _ => None,")
    lines.append("    }")
    lines.append("}")
    return lines


def snake_case(name: str) -> str:
    out: List[str] = []
    for idx, ch in enumerate(name):
        if ch.isupper() and idx > 0:
            prev = name[idx - 1]
            if prev.islower() or (idx + 1 < len(name) and name[idx + 1].islower()):
                out.append("_")
        out.append(ch.lower())
    return "".join(out)


def generate_response_parsers(responses: List[ResponseSpec]) -> List[str]:
    lines: List[str] = []
    for resp in responses:
        func_name = f"parse_{snake_case(resp.name)}_body"
        if resp.body.type_name == "list_u8":
            if not resp.body.item:
                raise ValueError("list_u8 response requires item")
            item = resp.body.item
            lines.append(f"pub fn {func_name}(body: &[u8]) -> Result<Vec<{item}>> {{")
            lines.append("    if body.is_empty() {")
            lines.append("        return Ok(Vec::new());")
            lines.append("    }")
            lines.append("    let mut cursor = RecordCursor::new(body, 0);")
            lines.append("    let count = cursor.take_u8()? as usize;")
            lines.append("    let mut out = Vec::with_capacity(count);")
            lines.append("    for _ in 0..count {")
            lines.append(f"        out.push({item}::decode(&mut cursor)?);")
            lines.append("    }")
            lines.append("    Ok(out)")
            lines.append("}")
            lines.append("")
        elif resp.body.type_name == "record":
            if not resp.body.item:
                raise ValueError("record response requires item")
            item = resp.body.item
            error_ctx = snake_case(resp.name).replace("_", " ")
            lines.append(f"pub fn {func_name}(body: &[u8]) -> Result<{item}> {{")
            lines.append("    if body.is_empty() {")
            lines.append(f"        return Err(RacError::Decode(\"{error_ctx} empty body\"));")
            lines.append("    }")
            lines.append("    let mut cursor = RecordCursor::new(body, 0);")
            lines.append(f"    {item}::decode(&mut cursor)")
            lines.append("}")
            lines.append("")
        else:
            raise ValueError(f"unknown response body type: {resp.body.type_name}")
    return lines


def render_value(value: Any, prefer_hex: bool = True) -> str:
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, int):
        if prefer_hex and value >= 10:
            return f"0x{value:x}"
        return str(value)
    if isinstance(value, str):
        return f"\"{value}\""
    if isinstance(value, list):
        parts = ", ".join(render_value(v, prefer_hex) for v in value)
        return f"[{parts}]"
    if isinstance(value, dict):
        raise ValueError("dict values require explicit formatting")
    raise ValueError("unsupported test value")


def generate_response_tests(responses: List[ResponseSpec]) -> List[str]:
    lines: List[str] = []
    tests = [t for r in responses for t in r.tests]
    if not tests:
        return lines
    lines.append("#[cfg(test)]")
    lines.append("mod tests {")
    lines.append("    use super::*;")
    lines.append("    use crate::commands::rpc_body;")
    lines.append("")
    lines.append("    fn decode_hex_str(input: &str) -> Vec<u8> {")
    lines.append("        hex::decode(input.trim()).expect(\"hex decode\")")
    lines.append("    }")
    lines.append("")

    for resp in responses:
        func_name = f"parse_{snake_case(resp.name)}_body"
        for test in resp.tests:
            test_name = snake_case(test.name)
            lines.append("    #[test]")
            lines.append(f"    fn {test_name}() {{")
            lines.append(f"        let hex = include_str!(\"{test.hex_path}\");")
            lines.append("        let payload = decode_hex_str(hex);")
            lines.append("        let body = rpc_body(&payload).expect(\"rpc body\");")
            if resp.body.type_name == "list_u8":
                lines.append(f"        let items = {func_name}(body).expect(\"parse body\");")
                if test.expect_len is not None:
                    lines.append(f"        assert_eq!(items.len(), {test.expect_len});")
                for assertion in test.asserts:
                    if assertion.index is None:
                        raise ValueError("list response asserts require index")
                    rendered = render_value(assertion.value)
                    lines.append(
                        f"        assert_eq!(items[{assertion.index}].{assertion.field}, {rendered});"
                    )
            else:
                lines.append(f"        let record = {func_name}(body).expect(\"parse body\");")
                for assertion in test.asserts:
                    if assertion.index is not None:
                        raise ValueError("record response asserts must not use index")
                    rendered = render_value(assertion.value)
                    lines.append(
                        f"        assert_eq!(record.{assertion.field}, {rendered});"
                    )
            lines.append("    }")
            lines.append("")
    lines.append("}")
    return lines


def generate_requests(requests: List[RequestSpec], include_uses: bool = True) -> str:
    lines: List[str] = []
    if include_uses:
        lines.extend(request_uses(requests))
        lines.append("")

    for req in requests:
        derive = ", ".join(req.derives)
        lines.append(f"#[derive({derive})]")
        lines.append(f"pub struct {req.name} {{")
        for field in req.fields:
            if field.skip or field.literal is not None:
                continue
            rust_ty = request_rust_type(field)
            lines.append(f"    pub {field.name}: {rust_ty},")
        lines.append("}")
        lines.append("")
        lines.append(f"impl {req.name} {{")
        lines.append("    pub fn encoded_len(&self) -> usize {")
        len_parts: List[str] = []
        for field in req.fields:
            if field.type_name == "str8":
                len_parts.append(f"1 + self.{field.name}.len()")
            else:
                len_parts.append(str(request_encoded_len(field)))
        if len_parts:
            lines.append(f"        { ' + '.join(len_parts) }")
        else:
            lines.append("        0")
        lines.append("    }")
        lines.append("")
        lines.append("    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {")
        for field in req.fields:
            if field.skip:
                continue
            exprs = request_encode_expr(field)
            for expr in exprs:
                lines.append(f"        {expr}")
        lines.append("        Ok(())")
        lines.append("    }")
        lines.append("}")
        lines.append("")

    return "\n".join(lines).rstrip() + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate RAC schema code from TOML schema")
    parser.add_argument("schema", help="Path to TOML schema")
    parser.add_argument("--out", help="Output .rs file path")
    args = parser.parse_args()

    schema_path = Path(args.schema)
    if not schema_path.is_absolute():
        schema_path = (ROOT / schema_path).resolve()

    records, requests, rpcs, responses = parse_schema(schema_path)
    extra_uses = request_uses(requests) if requests else None
    output = generate(records, responses, rpcs, extra_uses)
    if requests:
        output = output + "\n" + generate_requests(requests, include_uses=False)
    if responses:
        output = output + "\n" + "\n".join(generate_response_tests(responses)).rstrip() + "\n"

    out_path: Path
    if args.out:
        out_path = Path(args.out)
        if not out_path.is_absolute():
            out_path = (ROOT / out_path).resolve()
    else:
        out_path = schema_path.with_suffix(".generated.rs")

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

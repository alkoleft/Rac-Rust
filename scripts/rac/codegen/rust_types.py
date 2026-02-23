from typing import Dict, List

from .schema import FieldSpec, RecordSpec, RequestSpec, ResponseSpec


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


def needs_rac_error_responses(responses: List[ResponseSpec]) -> bool:
    for resp in responses:
        if resp.body.type_name == "record":
            return True
    return False

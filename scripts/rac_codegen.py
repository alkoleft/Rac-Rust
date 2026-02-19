#!/usr/bin/env python3
import argparse
try:
    import tomllib  # type: ignore
    _TOML_AVAILABLE = True
except ModuleNotFoundError:  # Python < 3.11
    _TOML_AVAILABLE = False
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, List, Optional


ROOT = Path(__file__).resolve().parents[1]


@dataclass
class FieldSpec:
    name: str
    type_name: str
    item: Optional[str] = None
    length: Optional[int] = None
    skip: bool = False
    computed: Optional[str] = None
    source: Optional[str] = None
    rust_type: Optional[str] = None


@dataclass
class RecordSpec:
    name: str
    derives: List[str]
    fields: List[FieldSpec]


def parse_schema(path: Path) -> List[RecordSpec]:
    if _TOML_AVAILABLE:
        payload = tomllib.loads(path.read_text())
        return parse_schema_payload(payload)
    return parse_schema_minimal(path)


def parse_schema_payload(payload: Dict[str, Any]) -> List[RecordSpec]:
    records: List[RecordSpec] = []
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
                    skip=bool(raw.get("skip", False)),
                    computed=raw.get("computed"),
                    source=raw.get("source"),
                    rust_type=raw.get("rust_type"),
                )
            )
        records.append(RecordSpec(name=name, derives=derives, fields=fields))
    return records


def parse_schema_minimal(path: Path) -> List[RecordSpec]:
    lines = path.read_text().splitlines()
    records: Dict[str, Dict[str, Any]] = {}
    current: Optional[Dict[str, Any]] = None
    current_name: Optional[str] = None
    in_fields = False
    fields_buf: List[Dict[str, Any]] = []

    for raw in lines:
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        if line.startswith("[record.") and line.endswith("]"):
            if current_name:
                current["fields"] = fields_buf
                records[current_name] = current
            current_name = line[len("[record.") : -1]
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
        if in_fields:
            if line.startswith("]"):
                in_fields = False
                continue
            if line.endswith(","):
                line = line[:-1].strip()
            if line.startswith("{") and line.endswith("}"):
                fields_buf.append(parse_inline_table(line))
            continue

    if current_name and current is not None:
        current["fields"] = fields_buf
        records[current_name] = current

    return parse_schema_payload({"record": records})


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
    if raw_val in {"true", "false"}:
        return raw_val == "true"
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
        if ch == sep and not in_str:
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
    if field.type_name in {"str8", "str_u14", "datetime_u64_be"}:
        return "String"
    if field.type_name == "bytes":
        return "Vec<u8>"
    if field.type_name == "u8":
        return "u8"
    if field.type_name == "u16_be":
        return "u16"
    if field.type_name == "u32_be":
        return "u32"
    if field.type_name == "u64_be":
        return "u64"
    if field.type_name == "f64_be":
        return "f64"
    if field.type_name in {"u8_bool", "u32_be_bool"}:
        return "bool"
    if field.type_name == "list_u8":
        if not field.item:
            raise ValueError("list_u8 requires item")
        return f"Vec<{field.item}>"
    if field.type_name == "computed":
        return field.rust_type or "bool"
    raise ValueError(f"unknown type: {field.type_name}")


def decode_expr(field: FieldSpec) -> List[str]:
    t = field.type_name
    if t == "uuid":
        return ["cursor.take_uuid()?;"]
    if t == "str8":
        return ["cursor.take_str8()?;"]
    if t == "str_u14":
        return [
            "let b0 = cursor.take_u8()? as usize;",
            "let b1 = cursor.take_u8()? as usize;",
            "let len = (b0 & 0x3f) | (b1 << 6);",
            "let bytes = cursor.take_bytes(len)?;",
            "String::from_utf8_lossy(&bytes).to_string();",
        ]
    if t == "bytes":
        if field.length is None:
            raise ValueError("bytes requires len")
        return [f"cursor.take_bytes({field.length})?;"]
    if t == "u8":
        return ["cursor.take_u8()?;"]
    if t == "u16_be":
        return ["cursor.take_u16_be()?;"]
    if t == "u32_be":
        return ["cursor.take_u32_be()?;"]
    if t == "u64_be":
        return ["cursor.take_u64_be()?;"]
    if t == "f64_be":
        return ["cursor.take_f64_be()?;"]
    if t == "u8_bool":
        return ["cursor.take_u8()? != 0;"]
    if t == "u32_be_bool":
        return ["cursor.take_u32_be()? != 0;"]
    if t == "datetime_u64_be":
        return ["v8_datetime_to_iso(cursor.take_u64_be()?).unwrap_or_default();"]
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
    raise ValueError(f"unknown type for decode: {t}")


def needs_datetime(records: List[RecordSpec]) -> bool:
    for record in records:
        for field in record.fields:
            if field.type_name == "datetime_u64_be":
                return True
    return False


def generate(records: List[RecordSpec]) -> str:
    lines: List[str] = []
    uses = ["use crate::codec::RecordCursor;", "use crate::error::Result;", "use crate::Uuid16;"]
    if needs_datetime(records):
        uses.insert(0, "use crate::codec::v8_datetime_to_iso;")
    uses.append("use serde::Serialize;")
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
        for field in record.fields:
            if field.computed:
                if field.computed != "ne_zero" or not field.source:
                    raise ValueError("computed fields must use computed='ne_zero' with source")
                computed_lines.append(f"        let {field.name} = {field.source} != 0;")
                continue
            var_name = field.name
            if field.skip:
                var_name = f"_{field.name}"
            expr = decode_expr(field)
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

    return "\n".join(lines).rstrip() + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate RAC record decoders from TOML schema")
    parser.add_argument("schema", help="Path to TOML schema")
    parser.add_argument("--out", help="Output .rs file path")
    args = parser.parse_args()

    schema_path = Path(args.schema)
    if not schema_path.is_absolute():
        schema_path = (ROOT / schema_path).resolve()

    records = parse_schema(schema_path)
    output = generate(records)

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

#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, List, Optional

try:
    import tomllib  # type: ignore
except ModuleNotFoundError:  # Python < 3.11
    try:
        import tomli as tomllib  # type: ignore
    except ModuleNotFoundError:
        tomllib = None  # type: ignore

try:
    from codegen.parse import parse_schema
except Exception:  # pragma: no cover - optional dependency in runtime
    parse_schema = None


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_SCHEMA = ROOT / "schemas" / "rac" / "console_output.toml"
DEFAULT_OUT = ROOT / "apps" / "rac_cli" / "src" / "rac_lite" / "console_output_generated.rs"
DEFAULT_SCHEMA_DIR = ROOT / "schemas" / "rac"

STRING_TYPES = {
    "str8",
    "str8_opt",
    "str8_default",
    "str_len_u8",
    "str_len_u8_or_2c",
    "str_u14",
    "datetime_u64_be",
    "datetime_u64_be_opt",
    "datetime_u64_be_default",
}
UUID_TYPES = {"uuid", "uuid_opt", "uuid_default"}
BOOL_TYPES = {"u8_bool", "u16_be_bool", "u32_be_bool", "bool_default", "bool_opt"}


@dataclass
class IfLetSpec:
    var: str
    expr: str


@dataclass
class LineSpec:
    fmt: Optional[str]
    args: List[str]
    label: Optional[str]
    value: Optional[str]
    format_name: Optional[str]
    optional: Optional[IfLetSpec]
    call: Optional[str]


@dataclass
class ListSpec:
    label: str
    fn_name: str
    struct_name: str
    style: str


@dataclass
class RecordSpec:
    base: str
    type_name: str
    info_fn: str
    info_struct: str
    list_specs: List[ListSpec]
    lines: List[LineSpec]
    label_align: Optional[int]


def snake_to_pascal(value: str) -> str:
    return "".join(part[:1].upper() + part[1:] for part in value.split("_") if part)


def rust_string_literal(value: str) -> str:
    escaped = value.replace("\\", "\\\\").replace("\"", "\\\"")
    return f"\"{escaped}\""


def parse_lines(raw_lines: List[Dict[str, Any]]) -> List[LineSpec]:
    lines: List[LineSpec] = []
    for raw in raw_lines:
        call = str(raw.get("call", "")).strip() if "call" in raw else None
        fmt_raw = raw.get("fmt")
        label_raw = raw.get("label")
        value_raw = raw.get("value")
        format_raw = raw.get("format")

        if call:
            lines.append(
                LineSpec(
                    fmt=None,
                    args=[],
                    label=None,
                    value=None,
                    format_name=None,
                    optional=None,
                    call=call,
                )
            )
            continue

        fmt = str(fmt_raw).strip() if fmt_raw is not None else None
        if fmt is not None and not fmt:
            raise ValueError("line fmt is empty")
        args = [str(arg) for arg in raw.get("args", []) or []]

        label = str(label_raw).strip() if label_raw is not None else None
        value = str(value_raw).strip() if value_raw is not None else None
        format_name = str(format_raw).strip() if format_raw is not None else None

        opt_raw = raw.get("optional") or raw.get("if_let")
        optional = None
        if opt_raw:
            if isinstance(opt_raw, dict):
                var = str(opt_raw.get("var", "value")).strip() or "value"
                expr = str(opt_raw.get("expr", "")).strip()
            else:
                var = "value"
                expr = str(opt_raw).strip()
            if not expr:
                raise ValueError("optional requires expr")
            optional = IfLetSpec(var=var, expr=expr)

        if fmt is None and not label and not value:
            raise ValueError("line requires fmt or label/value")

        if fmt is None:
            if not value and optional:
                value = optional.var
            if not value:
                raise ValueError("line value is required when fmt is not set")
            if optional and value == optional.var:
                pass
            elif re.match(r"^[A-Za-z_][A-Za-z0-9_]*$", value or ""):
                value = f"item.{value}"

        lines.append(
            LineSpec(
                fmt=fmt,
                args=args,
                label=label,
                value=value,
                format_name=format_name,
                optional=optional,
                call=None,
            )
        )
    return lines


def parse_lists(raw_lists: List[Dict[str, Any]]) -> List[ListSpec]:
    lists: List[ListSpec] = []
    for raw in raw_lists:
        label = str(raw.get("label", "")).strip()
        fn_name = str(raw.get("fn", "")).strip()
        struct_name = str(raw.get("struct", "")).strip()
        style = str(raw.get("style", "counted")).strip()
        if not label or not fn_name or not struct_name:
            raise ValueError("list requires label, fn, struct")
        lists.append(ListSpec(label=label, fn_name=fn_name, struct_name=struct_name, style=style))
    return lists


def parse_records(
    payload: Dict[str, Any],
    record_field_types: Dict[str, Dict[str, str]],
) -> List[RecordSpec]:
    record_table: Dict[str, Any] = payload.get("record", {})
    records: List[RecordSpec] = []
    for base, raw in record_table.items():
        type_name = str(raw.get("type", "")).strip()
        if not type_name:
            raise ValueError(f"record {base} missing type")
        pascal = snake_to_pascal(base)
        info_fn = str(raw.get("info_fn") or f"{base}_info")
        info_struct = str(raw.get("info_struct") or f"{pascal}InfoDisplay")
        label_align = raw.get("label_align")
        if label_align == "auto":
            label_align = -1
        elif isinstance(label_align, str):
            label_align = int(label_align)
        elif isinstance(label_align, int):
            label_align = label_align
        elif label_align is None:
            label_align = None
        else:
            raise ValueError(f"invalid label_align for {base}")
        lines = parse_lines(raw.get("lines", []) or [])

        list_specs: List[ListSpec] = []
        if raw.get("lists"):
            list_specs = parse_lists(raw.get("lists"))
        else:
            list_label = str(raw.get("list_label", "")).strip()
            if list_label:
                list_fn = str(raw.get("list_fn") or f"{base}_list")
                list_struct = str(raw.get("list_struct") or f"{pascal}ListDisplay")
                list_style = str(raw.get("list_style", "counted")).strip()
                list_specs = [
                    ListSpec(
                        label=list_label,
                        fn_name=list_fn,
                        struct_name=list_struct,
                        style=list_style,
                    )
                ]
        record = RecordSpec(
            base=base,
            type_name=type_name,
            info_fn=info_fn,
            info_struct=info_struct,
            list_specs=list_specs,
            lines=lines,
            label_align=label_align,
        )
        apply_label_align(record)
        apply_default_formats(record, record_field_types)
        records.append(record)
    return records


def is_value_complete(value: str) -> bool:
    in_str = False
    escaped = False
    depth = 0
    for ch in value:
        if escaped:
            escaped = False
            continue
        if ch == "\\" and in_str:
            escaped = True
            continue
        if ch == "\"":
            in_str = not in_str
            continue
        if in_str:
            continue
        if ch in "[{":
            depth += 1
            continue
        if ch in "]}":
            if depth > 0:
                depth -= 1
            continue
    return depth == 0 and not in_str


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


def unescape_string(value: str) -> str:
    out: List[str] = []
    escaped = False
    for ch in value:
        if escaped:
            out.append(ch)
            escaped = False
            continue
        if ch == "\\":
            escaped = True
            continue
        out.append(ch)
    return "".join(out)


def strip_quotes(value: str) -> str:
    if value.startswith("\"") and value.endswith("\""):
        return unescape_string(value[1:-1])
    return value


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


def parse_toml_minimal(path: Path) -> Dict[str, Any]:
    lines = path.read_text().splitlines()
    records: Dict[str, Any] = {}
    current: Optional[Dict[str, Any]] = None
    current_name: Optional[str] = None

    i = 0
    while i < len(lines):
        raw = lines[i]
        line = raw.strip()
        i += 1
        if not line or line.startswith("#"):
            continue
        if line.startswith("[record.") and line.endswith("]"):
            if current_name and current is not None:
                records[current_name] = current
            current_name = line[len("[record.") : -1]
            current = {}
            continue
        if current is None:
            continue
        if "=" in line:
            key, raw_val = line.split("=", 1)
            key = key.strip()
            raw_val = raw_val.strip()
            if not is_value_complete(raw_val) and raw_val.startswith(("[", "{")):
                parts = [raw_val]
                while i < len(lines):
                    next_line = lines[i].strip()
                    i += 1
                    if not next_line or next_line.startswith("#"):
                        continue
                    parts.append(next_line)
                    candidate = " ".join(parts)
                    if is_value_complete(candidate):
                        raw_val = candidate
                        break
                else:
                    raw_val = " ".join(parts)
            current[key] = parse_value(raw_val)
            continue

    if current_name and current is not None:
        records[current_name] = current

    return {"record": records}


def load_record_field_types(schema_dir: Path) -> Dict[str, Dict[str, str]]:
    record_types: Dict[str, Dict[str, str]] = {}
    if parse_schema is None:
        return record_types
    for path in sorted(schema_dir.glob("*.toml")):
        if path.name == "console_output.toml":
            continue
        records, _, _, _ = parse_schema(path)
        for record in records:
            fields = {field.name: field.type_name for field in record.fields}
            record_types[record.name] = fields
    return record_types


def infer_format_name(
    record_type: str,
    field_name: str,
    record_field_types: Dict[str, Dict[str, str]],
) -> str:
    field_type = record_field_types.get(record_type, {}).get(field_name)
    if not field_type:
        return "raw"
    if field_type in STRING_TYPES:
        return "display_str"
    if field_type in UUID_TYPES:
        return "uuid"
    if field_type in BOOL_TYPES:
        return "yes_no"
    return "raw"


def apply_default_formats(record: RecordSpec, record_field_types: Dict[str, Dict[str, str]]) -> None:
    for line in record.lines:
        if line.call or line.fmt or line.format_name:
            continue
        value = line.value or ""
        match = re.match(r"^item\.([A-Za-z_][A-Za-z0-9_]*)$", value)
        if not match:
            continue
        field_name = match.group(1)
        line.format_name = infer_format_name(record.type_name, field_name, record_field_types)


def apply_label_align(record: RecordSpec) -> None:
    if record.label_align != -1:
        return
    max_len = 0
    for line in record.lines:
        if line.label:
            max_len = max(max_len, len(line.label))
    record.label_align = max_len if max_len > 0 else None


def build_fmt_and_args(line: LineSpec, label_align: Optional[int]) -> tuple[str, List[str]]:
    if line.fmt:
        return line.fmt, line.args
    label = line.label or ""
    value = line.value or ""
    format_name = (line.format_name or "raw").strip()

    if format_name == "raw":
        fmt = "{}"
        args = [value]
    elif format_name == "display_str":
        fmt = "{}"
        args = [f"display_str(&{value})"]
    elif format_name == "uuid":
        fmt = "{}"
        args = [f"format_uuid(&{value})"]
    elif format_name == "yes_no":
        fmt = "{}"
        args = [f"yes_no({value})"]
    elif format_name == "quoted":
        fmt = "\"{}\""
        args = [value]
    elif format_name == "quoted_display":
        fmt = "\"{}\""
        args = [f"display_str(&{value})"]
    elif format_name == "float3":
        fmt = "{:.3}"
        args = [value]
    elif format_name == "hex_u32":
        fmt = "0x{:08x}"
        args = [value]
    elif format_name == "bytes3":
        fmt = "{:02x} {:02x} {:02x}"
        args = [f"{value}[0]", f"{value}[1]", f"{value}[2]"]
    else:
        raise ValueError(f"unknown format: {format_name}")

    if label:
        if label_align:
            padding = " " * max(label_align - len(label), 0)
            fmt = f"{label}{padding}: {fmt}"
        else:
            fmt = f"{label}: {fmt}"
    return fmt, args


def emit_line(buf: List[str], line: LineSpec, indent: str, label_align: Optional[int]) -> None:
    if line.call:
        call = line.call.rstrip(";")
        buf.append(f"{indent}{call};")
        return
    fmt, args_list = build_fmt_and_args(line, label_align)
    fmt_literal = rust_string_literal(fmt)
    args = ", " + ", ".join(args_list) if args_list else ""
    if line.optional:
        buf.append(f"{indent}if let Some({line.optional.var}) = {line.optional.expr} {{")
        buf.append(f"{indent}    outln!(out, {fmt_literal}{args});")
        buf.append(f"{indent}}}")
    else:
        buf.append(f"{indent}outln!(out, {fmt_literal}{args});")


def generate(records: List[RecordSpec], schema_path: Path) -> str:
    out: List[str] = []
    out.append("// @generated by scripts/rac/console_output_codegen.py. DO NOT EDIT.")
    out.append(f"// source: {schema_path}")
    out.append("")

    for record in records:
        base = record.base
        info_struct = record.info_struct
        info_fn = record.info_fn
        render_fn = f"render_{base}_info"
        type_name = record.type_name

        out.append(f"pub struct {info_struct}<'a> {{")
        out.append(f"    item: &'a {type_name},")
        out.append("}")
        out.append("")
        out.append(f"pub fn {info_fn}(item: &{type_name}) -> {info_struct}<'_> {{")
        out.append(f"    {info_struct} {{ item }}")
        out.append("}")
        out.append("")
        out.append(f"fn {render_fn}(out: &mut String, item: &{type_name}) {{")
        for line in record.lines:
            emit_line(out, line, "    ", record.label_align)
        out.append("}")
        out.append("")
        out.append(f"impl Display for {info_struct}<'_> {{")
        out.append("    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {")
        out.append("        let mut out = String::new();")
        out.append(f"        {render_fn}(&mut out, self.item);")
        out.append("        write_trimmed(f, &out)")
        out.append("    }")
        out.append("}")
        out.append("")

        for list_spec in record.list_specs:
            out.append(f"pub struct {list_spec.struct_name}<'a> {{")
            out.append(f"    items: &'a [{type_name}],")
            out.append("}")
            out.append("")
            out.append(
                f"pub fn {list_spec.fn_name}(items: &[{type_name}]) -> {list_spec.struct_name}<'_> {{"
            )
            out.append(f"    {list_spec.struct_name} {{ items }}")
            out.append("}")
            out.append("")
            out.append(f"impl Display for {list_spec.struct_name}<'_> {{")
            out.append("    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {")
            if list_spec.style == "blocks":
                out.append("        let mut out = String::new();")
                out.append("        for (idx, item) in self.items.iter().enumerate() {")
                out.append("            if idx > 0 {")
                out.append("                out.push('\\n');")
                out.append("            }")
                out.append(f"            {render_fn}(&mut out, item);")
                out.append("        }")
                out.append("        write_trimmed(f, &out)")
            else:
                out.append(
                    f"        let out = list_to_string({rust_string_literal(list_spec.label)}, self.items, 5, MoreLabel::Default, |out, _idx, item| {{"
                )
                out.append(f"            {render_fn}(out, item);")
                out.append("        });")
                out.append("        write_trimmed(f, &out)")
            out.append("    }")
            out.append("}")
            out.append("")

    return "\n".join(out).rstrip() + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate rac_lite console output helpers")
    parser.add_argument("--schema", default=str(DEFAULT_SCHEMA), help="Path to console output schema TOML")
    parser.add_argument("--out", default=str(DEFAULT_OUT), help="Output .rs file path")
    parser.add_argument(
        "--schema-dir",
        default=str(DEFAULT_SCHEMA_DIR),
        help="Directory with RAC protocol schemas (for type inference)",
    )
    args = parser.parse_args()

    schema_path = Path(args.schema)
    if not schema_path.is_absolute():
        schema_path = (ROOT / schema_path).resolve()
    schema_dir = Path(args.schema_dir)
    if not schema_dir.is_absolute():
        schema_dir = (ROOT / schema_dir).resolve()
    record_field_types = load_record_field_types(schema_dir)

    if tomllib is None:
        payload = parse_toml_minimal(schema_path)
    else:
        payload = tomllib.loads(schema_path.read_text())
    records = parse_records(payload, record_field_types)

    out_path = Path(args.out)
    if not out_path.is_absolute():
        out_path = (ROOT / out_path).resolve()
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(generate(records, schema_path))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

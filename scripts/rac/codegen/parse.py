from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

try:
    import tomllib  # type: ignore
    _TOML_AVAILABLE = True
except ModuleNotFoundError:  # Python < 3.11
    _TOML_AVAILABLE = False

from .schema import (
    FieldSpec,
    RecordSpec,
    RequestSpec,
    RpcSpec,
    RpcTestSpec,
    ResponseAssertSpec,
    ResponseBodySpec,
    ResponseSpec,
    ResponseTestSpec,
)


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
                response=spec.get("response"),
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
            tail_len_param=body_spec.get("tail_len_param"),
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
                    tail_len=raw.get("tail_len"),
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

    i = 0
    while i < len(lines):
        raw = lines[i]
        line = raw.strip()
        i += 1
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

    return parse_schema_payload(
        {"record": records, "request": requests, "rpc": rpcs, "response": responses}
    )


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

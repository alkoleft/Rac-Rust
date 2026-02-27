from typing import Any, Dict, List, Optional

from .schema import RecordSpec, RequestSpec, ResponseSpec, RpcSpec, Version
from .rust_types import (
    decode_expr,
    needs_datetime,
    needs_rac_error,
    needs_rac_error_responses,
    needs_uuid,
    request_encoded_len,
    request_encode_expr,
    request_rust_type,
    request_uses,
    rust_type,
)


def generate(
    records: List[RecordSpec],
    requests: List[RequestSpec],
    responses: List[ResponseSpec],
    rpcs: List[RpcSpec],
    extra_uses: Optional[List[str]] = None,
) -> str:
    lines: List[str] = []
    uses = ["use crate::codec::RecordCursor;", "use crate::error::Result;"]
    uses.insert(0, "use crate::protocol::ProtocolVersion;")
    if needs_datetime(records):
        uses.insert(0, "use crate::codec::v8_datetime_to_iso;")
    if needs_rac_error(records) or needs_rac_error_responses(responses) or rpcs:
        uses.insert(0, "use crate::error::RacError;")
    if needs_uuid(records):
        uses.insert(0, "use crate::Uuid16;")
    uses.append("use serde::Serialize;")
    if rpcs:
        req_specs = collect_request_specs(rpcs, requests)
        extra_req_uses = request_uses(req_specs)
        for item in extra_req_uses:
            if item not in uses:
                uses.append(item)
        if any(rpc.response for rpc in rpcs):
            pass
    if extra_uses:
        for item in extra_uses:
            if item not in uses:
                uses.append(item)
    lines.extend(uses)
    lines.append("")

    if rpcs:
        lines.extend(generate_rpc_method_consts(rpcs))

    for record in records:
        min_version = min(field.version.start for field in record.fields) if record.fields else None
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
        lines.append(
            "    pub fn decode(cursor: &mut RecordCursor<'_>, protocol_version: ProtocolVersion) -> Result<Self> {"
        )
        lines.append("        let _ = protocol_version;")

        computed_lines: List[str] = []
        var_map: Dict[str, str] = {}
        for field in record.fields:
            if field.computed:
                if not field.source:
                    raise ValueError("computed fields require source")
                guard = render_version_guard(field.version, min_version, "protocol_version")
                if field.optional:
                    computed_lines.append(f"        let {field.name} = if {guard} {{")
                    if field.computed == "ne_zero":
                        computed_lines.append(f"            Some({field.source} != 0)")
                    elif field.computed == "literal":
                        computed_lines.append(f"            Some({field.source})")
                    else:
                        raise ValueError(
                            "computed fields must use computed='ne_zero' or computed='literal'"
                        )
                    computed_lines.append("        } else {")
                    computed_lines.append("            None")
                    computed_lines.append("        };")
                else:
                    if field.computed == "ne_zero":
                        computed_lines.append(f"        let {field.name} = {field.source} != 0;")
                    elif field.computed == "literal":
                        computed_lines.append(f"        let {field.name} = {field.source};")
                    else:
                        raise ValueError(
                            "computed fields must use computed='ne_zero' or computed='literal'"
                        )
                continue
            var_name = field.name
            if field.skip:
                var_name = f"_{field.name}"
            var_map[field.name] = var_name
            expr = decode_expr(field, var_map, "protocol_version")
            guard = render_version_guard(field.version, min_version, "protocol_version")
            if field.skip:
                if guard:
                    lines.append(f"        if {guard} {{")
                    emit_decode_statement(lines, var_name, expr, "            ")
                    lines.append("        }")
                else:
                    emit_decode_statement(lines, var_name, expr, "        ")
            elif field.optional:
                lines.append(f"        let {var_name} = if {guard} {{")
                emit_decode_option(lines, expr, "            ")
                lines.append("        } else {")
                lines.append("            None")
                lines.append("        };")
            else:
                if guard:
                    lines.append(f"        if !{guard} {{")
                    lines.append(
                        f"            return Err(RacError::Unsupported(\"field {field.name} unsupported for protocol\"));"
                    )
                    lines.append("        }")
                emit_decode_statement(lines, var_name, expr, "        ")

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

    if rpcs:
        lines.extend(generate_rpc_section(rpcs, requests))
        lines.append("")

    if responses:
        lines.extend(generate_response_structs(responses, records))
        lines.append("")
        lines.extend(generate_response_parsers(responses))
        lines.append("")

    if rpcs:
        lines.extend(generate_rpc_metadata(rpcs))
        lines.append("")

    return "\n".join(lines).rstrip() + "\n"


def snake_case(name: str) -> str:
    out: List[str] = []
    for idx, ch in enumerate(name):
        if ch.isupper() and idx > 0:
            prev = name[idx - 1]
            if prev.islower() or (idx + 1 < len(name) and name[idx + 1].islower()):
                out.append("_")
        out.append(ch.lower())
    return "".join(out)


def protocol_version_const(version: Version) -> str:
    if version.minor != 0:
        raise ValueError(f"unsupported protocol minor version: {version}")
    return f"ProtocolVersion::V{version.major}_{version.minor}"


def render_version_guard(
    version_range: "VersionRange",
    min_version: Optional[Version],
    protocol_var: str,
) -> str:
    if min_version is not None:
        if version_range.start == min_version and version_range.end is None:
            return ""
    start = protocol_version_const(version_range.start)
    if version_range.end is None:
        return f"{protocol_var} >= {start}"
    end = protocol_version_const(version_range.end)
    return f"{protocol_var} >= {start} && {protocol_var} < {end}"


def emit_decode_statement(
    lines: List[str], var_name: str, expr: List[str], indent: str
) -> None:
    if len(expr) == 1:
        lines.append(f"{indent}let {var_name} = {expr[0]}")
        return
    lines.append(f"{indent}let {var_name} = {{")
    for step in expr[:-1]:
        lines.append(f"{indent}    {step}")
    last = expr[-1].rstrip(";")
    lines.append(f"{indent}    {last}")
    lines.append(f"{indent}}};")


def emit_decode_option(lines: List[str], expr: List[str], indent: str) -> None:
    if len(expr) == 1:
        lines.append(f"{indent}Some({expr[0].rstrip(';')})")
        return
    lines.append(f"{indent}Some({{")
    for step in expr[:-1]:
        lines.append(f"{indent}    {step}")
    last = expr[-1].rstrip(";")
    lines.append(f"{indent}    {last}")
    lines.append(f"{indent}}})")


def generate_response_parsers(responses: List[ResponseSpec]) -> List[str]:
    lines: List[str] = []
    for resp in responses:
        func_name = f"parse_{snake_case(resp.name)}_body"
        if resp.body.type_name == "record":
            if not resp.body.item:
                raise ValueError("record response requires item")
            item = resp.body.item
            error_ctx = snake_case(resp.name).replace("_", " ")
            lines.append(
                f"pub fn {func_name}(body: &[u8], protocol_version: ProtocolVersion) -> Result<{item}> {{"
            )
            lines.append("    if body.is_empty() {")
            lines.append(f"        return Err(RacError::Decode(\"{error_ctx} empty body\"));")
            lines.append("    }")
            lines.append("    let mut cursor = RecordCursor::new(body);")
            lines.append(f"    {item}::decode(&mut cursor, protocol_version)")
            lines.append("}")
            lines.append("")
        elif resp.body.type_name == "record_tail":
            if not resp.body.item:
                raise ValueError("record_tail response requires item")
            if not resp.body.tail_len_param:
                raise ValueError("record_tail response requires tail_len_param")
            item = resp.body.item
            param = resp.body.tail_len_param
            error_ctx = snake_case(resp.name).replace("_", " ")
            lines.append(
                f"pub fn {func_name}(body: &[u8], {param}: usize, protocol_version: ProtocolVersion) -> Result<{item}> {{"
            )
            lines.append("    if body.is_empty() {")
            lines.append(f"        return Err(RacError::Decode(\"{error_ctx} empty body\"));")
            lines.append("    }")
            lines.append("    let mut cursor = RecordCursor::new(body);")
            lines.append(f"    let record = {item}::decode(&mut cursor, protocol_version)?;")
            lines.append("    if " + param + " != 0 {")
            lines.append("        let _tail = cursor.take_bytes(" + param + ")?;")
            lines.append("    }")
            lines.append("    Ok(record)")
            lines.append("}")
            lines.append("")
        elif resp.body.type_name in {"list_u8", "list_u8_tail"}:
            continue
        else:
            raise ValueError(f"unknown response body type: {resp.body.type_name}")
    return lines


def generate_response_structs(
    responses: List[ResponseSpec], records: List[RecordSpec]
) -> List[str]:
    lines: List[str] = []
    record_map = {record.name: record for record in records}
    for resp in responses:
        if not resp.body.make_struct:
            continue
        if resp.body.type_name not in {"list_u8", "record"}:
            continue
        resp_name = f"{resp.name}Resp"
        if resp.body.type_name == "list_u8":
            if not resp.body.item:
                raise ValueError("list_u8 response requires item")
            item = resp.body.item
            field_name = resp.body.field_name or "items"
            lines.append("#[derive(Debug, Serialize)]")
            lines.append(f"pub struct {resp_name} {{")
            lines.append(f"    pub {field_name}: Vec<{item}>,")
            lines.append("}")
            lines.append("")
            lines.append(f"impl crate::rpc::Response for {resp_name} {{")
            lines.append(
                "    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {"
            )
            lines.append("        let body = crate::rpc::decode_utils::rpc_body(payload)?;")
            lines.append("        let protocol_version = _codec.protocol_version();")
            lines.append("        Ok(Self {")
            lines.append(
                f"            {field_name}: crate::commands::parse_list_u8(body, |cursor| {item}::decode(cursor, protocol_version))?,"
            )
            lines.append("        })")
            lines.append("    }")
            lines.append("}")
            lines.append("")
            continue

        if resp.body.type_name == "record":
            if not resp.body.item:
                raise ValueError("record response requires item")
            item = resp.body.item
            field_name = resp.body.field_name or "record"
            record_spec = record_map.get(item)
            field_spec = None
            if record_spec:
                for field in record_spec.fields:
                    if field.skip:
                        continue
                    if field.name == field_name:
                        field_spec = field
                        break
            field_type = item if field_spec is None else rust_type(field_spec)
            lines.append("#[derive(Debug, Serialize)]")
            lines.append(f"pub struct {resp_name} {{")
            lines.append(f"    pub {field_name}: {field_type},")
            lines.append("}")
            lines.append("")
            lines.append(f"impl crate::rpc::Response for {resp_name} {{")
            lines.append(
                "    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {"
            )
            lines.append("        let body = crate::rpc::decode_utils::rpc_body(payload)?;")
            lines.append("        let protocol_version = _codec.protocol_version();")
            lines.append(
                f"        let record = parse_{snake_case(resp.name)}_body(body, protocol_version)?;"
            )
            lines.append("        Ok(Self {")
            if field_spec is None:
                lines.append(f"            {field_name}: record,")
            else:
                lines.append(f"            {field_name}: record.{field_name},")
            lines.append("        })")
            lines.append("    }")
            lines.append("}")
            lines.append("")
            continue
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
    lines.append("#[cfg(all(test, feature = \"artifacts\"))]")
    lines.append("mod tests {")
    lines.append("    use super::*;")
    lines.append("    use crate::commands::rpc_body;")
    lines.append("    use crate::protocol::ProtocolVersion;")
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
            lines.append("        let protocol_version = ProtocolVersion::V16_0;")
            if resp.body.type_name == "list_u8":
                if not resp.body.item:
                    raise ValueError("list_u8 response requires item")
                item = resp.body.item
                lines.append(
                    f"        let items = crate::commands::parse_list_u8(body, |cursor| {item}::decode(cursor, protocol_version))"
                    ".expect(\"parse body\");"
                )
                if test.expect_len is not None:
                    lines.append(f"        assert_eq!(items.len(), {test.expect_len});")
                for assertion in test.asserts:
                    if assertion.index is None:
                        raise ValueError("list response asserts require index")
                    rendered = render_value(assertion.value)
                    lines.append(
                        f"        assert_eq!(items[{assertion.index}].{assertion.field}, {rendered});"
                    )
            elif resp.body.type_name == "list_u8_tail":
                if not resp.body.item:
                    raise ValueError("list_u8_tail response requires item")
                if not resp.body.tail_len_param:
                    raise ValueError("list_u8_tail response requires tail_len_param")
                item = resp.body.item
                tail_len = 0 if test.tail_len is None else test.tail_len
                lines.append(
                    f"        let items = crate::commands::parse_list_u8_tail(body, {tail_len}, |cursor| {item}::decode(cursor, protocol_version))"
                    ".expect(\"parse body\");"
                )
                if test.expect_len is not None:
                    lines.append(f"        assert_eq!(items.len(), {test.expect_len});")
                for assertion in test.asserts:
                    if assertion.index is None:
                        raise ValueError("list response asserts require index")
                    rendered = render_value(assertion.value)
                    lines.append(
                        f"        assert_eq!(items[{assertion.index}].{assertion.field}, {rendered});"
                    )
            elif resp.body.type_name == "record_tail":
                tail_len = 0 if test.tail_len is None else test.tail_len
                lines.append(
                    f"        let record = {func_name}(body, {tail_len}, protocol_version).expect(\"parse body\");"
                )
                for assertion in test.asserts:
                    if assertion.index is not None:
                        raise ValueError("record response asserts must not use index")
                    rendered = render_value(assertion.value)
                    lines.append(f"        assert_eq!(record.{assertion.field}, {rendered});")
            else:
                lines.append(
                    f"        let record = {func_name}(body, protocol_version).expect(\"parse body\");"
                )
                for assertion in test.asserts:
                    if assertion.index is not None:
                        raise ValueError("record response asserts must not use index")
                    rendered = render_value(assertion.value)
                    lines.append(f"        assert_eq!(record.{assertion.field}, {rendered});")
            lines.append("    }")
            lines.append("")
    lines.append("}")
    return lines


def generate_rpc_metadata(rpcs: List[RpcSpec]) -> List[str]:
    lines: List[str] = []
    for rpc in rpcs:
        const_name = f"RPC_{snake_case(rpc.name).upper()}_META"
        method_req = rpc_method_req_const(rpc.name)
        method_resp = (
            "None"
            if rpc.method_resp is None
            else f"Some({rpc_method_resp_const(rpc.name)})"
        )
        lines.append(f"pub const {const_name}: crate::rpc::Meta = crate::rpc::Meta {{")
        lines.append(f"    method_req: {method_req},")
        lines.append(f"    method_resp: {method_resp},")
        lines.append(f"    requires_cluster_context: {str(rpc.requires_cluster_context).lower()},")
        lines.append(f"    requires_infobase_context: {str(rpc.requires_infobase_context).lower()},")
        lines.append("};")
        lines.append("")
    return lines


def rpc_method_const_base(rpc_name: str) -> str:
    return f"METHOD_{snake_case(rpc_name).upper()}"


def rpc_method_req_const(rpc_name: str) -> str:
    return f"{rpc_method_const_base(rpc_name)}_REQ"


def rpc_method_resp_const(rpc_name: str) -> str:
    return f"{rpc_method_const_base(rpc_name)}_RESP"


def render_method_code(value: int) -> str:
    return f"0x{value:02x}"


def generate_rpc_method_consts(rpcs: List[RpcSpec]) -> List[str]:
    lines: List[str] = []
    for rpc in rpcs:
        base = rpc_method_const_base(rpc.name)
        lines.append(f"pub const {base}_REQ: u8 = {render_method_code(rpc.method_req)};")
        if rpc.method_resp is not None:
            lines.append(f"pub const {base}_RESP: u8 = {render_method_code(rpc.method_resp)};")
    if lines:
        lines.append("")
    return lines


def generate_requests(requests: List[RequestSpec], include_uses: bool = True) -> str:
    lines: List[str] = []
    if include_uses:
        lines.extend(request_uses(requests))
        if "use crate::protocol::ProtocolVersion;" not in lines:
            lines.insert(0, "use crate::protocol::ProtocolVersion;")
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
        lines.append("    pub fn encoded_len(&self, protocol_version: ProtocolVersion) -> usize {")
        len_parts: List[str] = []
        for field in req.fields:
            guard = render_version_guard(field.version, None, "protocol_version")
            len_expr = (
                f"1 + self.{field.name}.len()"
                if field.type_name == "str8"
                else str(request_encoded_len(field))
            )
            if guard:
                len_parts.append(f"if {guard} {{ {len_expr} }} else {{ 0 }}")
            else:
                len_parts.append(len_expr)
        if len_parts:
            lines.append(f"        { ' + '.join(len_parts) }")
        else:
            lines.append("        0")
        lines.append("    }")
        lines.append("")
        expr_lines: List[str] = []
        for field in req.fields:
            if field.skip:
                continue
            guard = render_version_guard(field.version, None, "protocol_version")
            exprs = request_encode_expr(field)
            if guard:
                expr_lines.append(f"        if {guard} {{")
                for expr in exprs:
                    expr_lines.append(f"            {expr}")
                expr_lines.append("        }")
            else:
                for expr in exprs:
                    expr_lines.append(f"        {expr}")
        out_name = "out" if expr_lines else "_out"
        lines.append(
            f"    pub fn encode_body(&self, {out_name}: &mut Vec<u8>, protocol_version: ProtocolVersion) -> Result<()> {{"
        )
        lines.extend(expr_lines)
        lines.append("        Ok(())")
        lines.append("    }")
        lines.append("}")
        lines.append("")

    return "\n".join(lines).rstrip() + "\n"


def generate_rpc_section(rpcs: List[RpcSpec], requests: List[RequestSpec]) -> List[str]:
    request_map = {req.name: req for req in requests}
    lines: List[str] = []
    emitted_requests = set()

    for rpc in rpcs:
        req_spec = None
        if rpc.request_inline is not None:
            req_spec = rpc.request_inline
        elif rpc.request:
            req_spec = request_map.get(rpc.request)
        if req_spec and rpc.request_inline is None and req_spec.name not in emitted_requests:
            lines.extend(render_request(req_spec))
            lines.append("")
            emitted_requests.add(req_spec.name)
        if not rpc.response:
            continue
        struct_name = f"{rpc.name}Rpc"
        fields = []
        if req_spec:
            for field in req_spec.fields:
                if field.skip or field.literal is not None:
                    continue
                fields.append((field.name, request_rust_type(field)))

        if fields:
            lines.append(f"pub struct {struct_name} {{")
            for name, ty in fields:
                lines.append(f"    pub {name}: {ty},")
            lines.append("}")
        else:
            lines.append(f"pub struct {struct_name};")
        lines.append("")
        response_ty = rpc.response
        if response_ty == "AckResponse":
            response_ty = "crate::rpc::AckResponse"
        lines.append(f"impl crate::rpc::Request for {struct_name} {{")
        lines.append(f"    type Response = {response_ty};")
        lines.append("")
        lines.append("    fn meta(&self) -> crate::rpc::Meta {")
        lines.append(f"        RPC_{snake_case(rpc.name).upper()}_META")
        lines.append("    }")
        lines.append("")
        lines.append("    fn cluster(&self) -> Option<crate::Uuid16> {")
        if rpc.requires_cluster_context or rpc.requires_infobase_context:
            if fields and any(name == "cluster" for name, _ in fields):
                lines.append("        Some(self.cluster)")
            else:
                lines.append("        None")
        else:
            lines.append("        None")
        lines.append("    }")
        lines.append("")
        lines.append(
            "    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {"
        )
        lines.append("        let protocol_version = _codec.protocol_version();")
        lines.append(
            f"        if !({render_version_guard(rpc.version, None, 'protocol_version')}) {{"
        )
        lines.append(
            f"            return Err(RacError::Unsupported(\"rpc {rpc.name} unsupported for protocol\"));"
        )
        lines.append("        }")
        if req_spec:
            if rpc.request_inline is not None:
                len_expr = request_len_expr(req_spec, "protocol_version")
                lines.append(f"        let mut out = Vec::with_capacity({len_expr});")
                for field in req_spec.fields:
                    if field.skip:
                        continue
                    guard = render_version_guard(field.version, None, "protocol_version")
                    exprs = request_encode_expr(field)
                    if guard:
                        lines.append(f"        if {guard} {{")
                        for expr in exprs:
                            lines.append(f"            {expr}")
                        lines.append("        }")
                    else:
                        for expr in exprs:
                            lines.append(f"        {expr}")
                if not req_spec.fields:
                    lines.append("        let _ = &mut out;")
                lines.append("        Ok(out)")
            else:
                lines.append(f"        let req = {req_spec.name} {{")
                for name, _ in fields:
                    lines.append(f"            {name}: self.{name}.clone(),")
                lines.append("        };")
                lines.append(
                    "        let mut out = Vec::with_capacity(req.encoded_len(protocol_version));"
                )
                lines.append("        req.encode_body(&mut out, protocol_version)?;")
                lines.append("        Ok(out)")
        else:
            lines.append("        Ok(Vec::new())")
        lines.append("    }")
        lines.append("}")
        lines.append("")

    return lines


def render_request(req: RequestSpec) -> List[str]:
    lines: List[str] = []
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
    lines.append("    pub fn encoded_len(&self, protocol_version: ProtocolVersion) -> usize {")
    len_parts: List[str] = []
    for field in req.fields:
        guard = render_version_guard(field.version, None, "protocol_version")
        len_expr = (
            f"1 + self.{field.name}.len()"
            if field.type_name == "str8"
            else str(request_encoded_len(field))
        )
        if guard:
            len_parts.append(f"if {guard} {{ {len_expr} }} else {{ 0 }}")
        else:
            len_parts.append(len_expr)
    if len_parts:
        lines.append(f"        { ' + '.join(len_parts) }")
    else:
        lines.append("        0")
    lines.append("    }")
    lines.append("")
    expr_lines: List[str] = []
    for field in req.fields:
        if field.skip:
            continue
        guard = render_version_guard(field.version, None, "protocol_version")
        exprs = request_encode_expr(field)
        if guard:
            expr_lines.append(f"        if {guard} {{")
            for expr in exprs:
                expr_lines.append(f"            {expr}")
            expr_lines.append("        }")
        else:
            for expr in exprs:
                expr_lines.append(f"        {expr}")
    out_name = "out" if expr_lines else "_out"
    lines.append(
        f"    pub fn encode_body(&self, {out_name}: &mut Vec<u8>, protocol_version: ProtocolVersion) -> Result<()> {{"
    )
    lines.extend(expr_lines)
    lines.append("        Ok(())")
    lines.append("    }")
    lines.append("}")
    return lines


def request_len_expr(req: RequestSpec, protocol_var: str) -> str:
    len_parts: List[str] = []
    for field in req.fields:
        guard = render_version_guard(field.version, None, protocol_var)
        len_expr = (
            f"1 + self.{field.name}.len()"
            if field.type_name == "str8"
            else str(request_encoded_len(field))
        )
        if guard:
            len_parts.append(f"if {guard} {{ {len_expr} }} else {{ 0 }}")
        else:
            len_parts.append(len_expr)
    if len_parts:
        return " + ".join(len_parts)
    return "0"


def collect_request_specs(rpcs: List[RpcSpec], requests: List[RequestSpec]) -> List[RequestSpec]:
    request_map = {req.name: req for req in requests}
    out: List[RequestSpec] = list(requests)
    seen = {req.name for req in out}
    for rpc in rpcs:
        if rpc.request_inline is not None and rpc.request_inline.name not in seen:
            out.append(rpc.request_inline)
            seen.add(rpc.request_inline.name)
        elif rpc.request and rpc.request in request_map and rpc.request not in seen:
            out.append(request_map[rpc.request])
            seen.add(rpc.request)
    return out

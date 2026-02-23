from typing import Any, Dict, List, Optional

from .schema import RecordSpec, RequestSpec, ResponseSpec, RpcSpec
from .rust_types import (
    decode_expr,
    needs_datetime,
    needs_protocol_version,
    needs_rac_error,
    needs_rac_error_responses,
    needs_uuid,
    request_encoded_len,
    request_encode_expr,
    request_rust_type,
    request_uses,
)


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
    if rpcs:
        uses.append("use crate::metadata::RpcMethodMeta;")
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
                rust_ty = request_rust_type(field)
                lines.append(f"    pub {field.name}: {rust_ty},")
                continue
            rust_ty = request_rust_type(field)
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
                    raise ValueError(
                        "computed fields must use computed='ne_zero' or computed='literal'"
                    )
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

    if rpcs:
        lines.extend(generate_rpc_metadata(rpcs))
        lines.append("")

    if responses:
        lines.extend(generate_response_parsers(responses))
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


def generate_response_parsers(responses: List[ResponseSpec]) -> List[str]:
    lines: List[str] = []
    for resp in responses:
        func_name = f"parse_{snake_case(resp.name)}_body"
        if resp.body.type_name == "record":
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
        elif resp.body.type_name == "record_tail":
            if not resp.body.item:
                raise ValueError("record_tail response requires item")
            if not resp.body.tail_len_param:
                raise ValueError("record_tail response requires tail_len_param")
            item = resp.body.item
            param = resp.body.tail_len_param
            error_ctx = snake_case(resp.name).replace("_", " ")
            lines.append(
                f"pub fn {func_name}(body: &[u8], {param}: usize) -> Result<{item}> {{"
            )
            lines.append("    if body.is_empty() {")
            lines.append(f"        return Err(RacError::Decode(\"{error_ctx} empty body\"));")
            lines.append("    }")
            lines.append("    let mut cursor = RecordCursor::new(body, 0);")
            lines.append(f"    let record = {item}::decode(&mut cursor)?;")
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
                if not resp.body.item:
                    raise ValueError("list_u8 response requires item")
                item = resp.body.item
                lines.append(
                    f"        let items = crate::commands::parse_list_u8(body, {item}::decode)"
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
                    f"        let items = crate::commands::parse_list_u8_tail(body, {tail_len}, {item}::decode)"
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
                    f"        let record = {func_name}(body, {tail_len}).expect(\"parse body\");"
                )
                for assertion in test.asserts:
                    if assertion.index is not None:
                        raise ValueError("record response asserts must not use index")
                    rendered = render_value(assertion.value)
                    lines.append(f"        assert_eq!(record.{assertion.field}, {rendered});")
            else:
                lines.append(f"        let record = {func_name}(body).expect(\"parse body\");")
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
        method_resp = "None" if rpc.method_resp is None else f"Some({rpc.method_resp})"
        lines.append(f"pub const {const_name}: RpcMethodMeta = RpcMethodMeta {{")
        lines.append(f"    method_req: {rpc.method_req},")
        lines.append(f"    method_resp: {method_resp},")
        lines.append(f"    requires_cluster_context: {str(rpc.requires_cluster_context).lower()},")
        lines.append(f"    requires_infobase_context: {str(rpc.requires_infobase_context).lower()},")
        lines.append("};")
        lines.append("")
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

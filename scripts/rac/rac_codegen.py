#!/usr/bin/env python3
import argparse
from pathlib import Path

from codegen.parse import parse_schema
from codegen.render import generate, generate_requests, generate_response_tests
from codegen.rust_types import request_uses

ROOT = Path(__file__).resolve().parents[1]


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

#!/usr/bin/env python3
import argparse
import subprocess
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_SCHEMA_DIR = REPO_ROOT / "schemas" / "rac"
DEFAULT_OUT_DIR = REPO_ROOT / "apps" / "rac_protocol" / "src" / "commands"
CODEGEN = REPO_ROOT / "scripts" / "rac" / "rac_codegen.py"


def resolve_path(path: str | Path, base: Path) -> Path:
    candidate = Path(path)
    if candidate.is_absolute():
        return candidate
    return (base / candidate).resolve()


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate all RAC command classes from TOML schemas")
    parser.add_argument("--schema-dir", default=str(DEFAULT_SCHEMA_DIR), help="Directory with RAC TOML schemas")
    parser.add_argument("--out-dir", default=str(DEFAULT_OUT_DIR), help="Output directory for *_generated.rs")
    parser.add_argument("--pattern", default="*.toml", help="Glob pattern for schema files")
    args = parser.parse_args()

    schema_dir = resolve_path(args.schema_dir, REPO_ROOT)
    out_dir = resolve_path(args.out_dir, REPO_ROOT)
    schemas = sorted(schema_dir.glob(args.pattern))

    if not schemas:
        print(f"No schemas found in {schema_dir} with pattern {args.pattern}", file=sys.stderr)
        return 1

    for schema in schemas:
        out_path = out_dir / f"{schema.stem}_generated.rs"
        subprocess.run(
            [sys.executable, str(CODEGEN), str(schema), "--out", str(out_path)],
            check=True,
        )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())

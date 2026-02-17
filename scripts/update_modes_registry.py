#!/usr/bin/env python3
import argparse
from pathlib import Path
from typing import List, Tuple


ROOT = Path(__file__).resolve().parents[1]


def split_row(line: str) -> List[str]:
    return [cell.strip() for cell in line.strip().strip("|").split("|")]


def join_row(cells: List[str]) -> str:
    return "| " + " | ".join(cells) + " |"


def normalize_mode(cell: str) -> str:
    return cell.strip().strip("`")


def parse_command_words(tokens: List[str]) -> Tuple[str, str]:
    if not tokens:
        raise ValueError("empty command")
    mode = tokens[0]
    words = []
    for tok in tokens[1:]:
        if tok.startswith("-"):
            break
        words.append(tok)
    if not words:
        raise ValueError("missing command words after mode")
    return mode, " ".join(words)


def update_registry(path: Path, mode: str, command: str, analyzed: str, implemented: str) -> bool:
    lines = path.read_text().splitlines()
    out = []
    in_table = False
    current_mode = ""
    updated = False

    for line in lines:
        if line.strip().startswith("| Mode") and "Command" in line:
            in_table = True
            out.append(line)
            continue
        if in_table and line.strip().startswith("|---"):
            out.append(line)
            continue
        if in_table and line.strip().startswith("|"):
            cells = split_row(line)
            if len(cells) < 7:
                out.append(line)
                continue
            if cells[0]:
                current_mode = normalize_mode(cells[0])
            if current_mode == mode and cells[3].strip("`") == command:
                if analyzed:
                    cells[4] = analyzed
                if implemented:
                    cells[5] = implemented
                line = join_row(cells)
                updated = True
            out.append(line)
            continue
        out.append(line)

    if updated:
        path.write_text("\n".join(out) + "\n")
    return updated


def main() -> None:
    parser = argparse.ArgumentParser(description="Update RAC modes registry table.")
    parser.add_argument("--mode", required=True, help="Mode name (e.g. server).")
    parser.add_argument("--command", required=True, help="Command words (e.g. list or summary list).")
    parser.add_argument("--analyzed", default="yes", help="Analyzed value (default: yes).")
    parser.add_argument("--implemented", default="", help="Implemented value (optional).")
    parser.add_argument("--file", default="docs/modes/rac_modes_registry.md", help="Registry file.")
    args = parser.parse_args()

    path = ROOT / args.file
    if not path.exists():
        raise SystemExit(f"Registry file not found: {path}")

    updated = update_registry(path, args.mode, args.command, args.analyzed, args.implemented)
    if not updated:
        raise SystemExit("No matching row found to update.")


if __name__ == "__main__":
    main()

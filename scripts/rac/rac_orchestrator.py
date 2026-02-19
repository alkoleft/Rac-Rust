#!/usr/bin/env python3
import argparse
import binascii
import json
import os
import re
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Dict, List, Optional, Tuple


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_CODEX_BIN = os.environ.get("CODEX_BIN", "codex")
DEFAULT_CODEX_FLAGS = os.environ.get("CODEX_FLAGS", "--full-auto")


@dataclass
class PipelineState:
    command: str
    capture_name: str
    doc_path: str
    session_dir: Optional[str] = None
    artifacts_dir: str = "artifacts"
    rac_out: Optional[str] = None
    rac_err: Optional[str] = None
    c2s_stream: Optional[str] = None
    s2c_stream: Optional[str] = None
    c2s_decode: Optional[str] = None
    s2c_decode: Optional[str] = None
    response_hex: Optional[str] = None
    score: Optional[float] = None
    score_reasons: List[str] = field(default_factory=list)
    needs_user: bool = False
    user_answers: Dict[str, str] = field(default_factory=dict)
    codex_last_message: Optional[str] = None


def run_cmd(
    cmd: List[str],
    cwd: Path,
    capture_output: bool = True,
    input_text: Optional[str] = None,
    env: Optional[Dict[str, str]] = None,
) -> subprocess.CompletedProcess:
    return subprocess.run(
        cmd,
        cwd=str(cwd),
        capture_output=capture_output,
        text=True,
        input=input_text,
        env=env,
        check=False,
    )


def write_state(state_path: Path, state: PipelineState) -> None:
    state_path.parent.mkdir(parents=True, exist_ok=True)
    state_path.write_text(json.dumps(asdict(state), indent=2, ensure_ascii=False) + "\n")


def parse_capture_output(output: str) -> Dict[str, str]:
    result = {}
    for line in output.splitlines():
        if "=" in line:
            key, value = line.split("=", 1)
            result[key.strip()] = value.strip()
    return result


def load_env_file(path: Path) -> Dict[str, str]:
    env = {}
    if not path.exists():
        return env
    for line in path.read_text().splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        env[key.strip()] = value.strip()
    return env


def substitute_command(template: str, env: Dict[str, str]) -> str:
    def repl(match: re.Match[str]) -> str:
        key = match.group(1)
        return env.get(key, match.group(0))

    return re.sub(r"\{([^}]+)\}", repl, template)


def run_capture(state: PipelineState, env: Dict[str, str]) -> None:
    cmd = [
        str(ROOT / "scripts" / "capture_rac_command.sh"),
        state.capture_name,
    ] + state.command.split()
    result = run_cmd(cmd, ROOT, env=env)
    if result.returncode != 0:
        raise RuntimeError(f"capture failed: {result.stderr.strip()}")
    meta = parse_capture_output(result.stdout)
    state.session_dir = meta.get("session_dir")
    state.rac_out = meta.get("rac_out")
    state.rac_err = meta.get("rac_err")
    if not state.session_dir:
        raise RuntimeError("capture did not return session_dir")


def run_decode(state: PipelineState) -> None:
    session_path = ROOT / "logs" / state.session_dir
    c2s = session_path / "client_to_server.stream.bin"
    s2c = session_path / "server_to_client.stream.bin"
    if not c2s.exists() or not s2c.exists():
        raise FileNotFoundError(f"missing stream files in {session_path}")
    state.c2s_stream = str(c2s)
    state.s2c_stream = str(s2c)
    artifacts = ROOT / state.artifacts_dir
    artifacts.mkdir(parents=True, exist_ok=True)
    c2s_out = artifacts / f"{state.capture_name}_client_to_server.decode.txt"
    s2c_out = artifacts / f"{state.capture_name}_server_to_client.decode.txt"
    cmd_base = ["cargo", "run", "-p", "rac_protocol", "--quiet", "--bin", "rac_decode", "--"]
    c2s_run = run_cmd(cmd_base + [str(c2s)], ROOT)
    s2c_run = run_cmd(cmd_base + [str(s2c)], ROOT)
    c2s_out.write_text(c2s_run.stdout)
    s2c_out.write_text(s2c_run.stdout)
    state.c2s_decode = str(c2s_out)
    state.s2c_decode = str(s2c_out)


def extract_artifacts(state: PipelineState) -> None:
    artifacts = ROOT / state.artifacts_dir
    artifacts.mkdir(parents=True, exist_ok=True)
    response_hex = artifacts / f"{state.capture_name}_response.hex"
    if state.s2c_stream:
        data = Path(state.s2c_stream).read_bytes()
        response_hex.write_text(binascii.hexlify(data).decode("ascii"))
        state.response_hex = str(response_hex)
    if state.rac_out:
        rac_out_path = Path(state.rac_out)
        if rac_out_path.exists():
            target = artifacts / f"{state.capture_name}_rac.out"
            target.write_text(rac_out_path.read_text(errors="replace"))


def codex_exec(prompt: str, state: PipelineState) -> None:
    artifacts = ROOT / state.artifacts_dir
    artifacts.mkdir(parents=True, exist_ok=True)
    last_message = artifacts / f"{state.capture_name}_codex_last.md"
    cmd = [
        DEFAULT_CODEX_BIN,
        "exec",
        "-",
        "--cd",
        str(ROOT),
    ] + DEFAULT_CODEX_FLAGS.split() + [
        "--output-last-message",
        str(last_message),
    ]
    result = run_cmd(cmd, ROOT, capture_output=True, input_text=prompt)
    if result.returncode != 0:
        raise RuntimeError(f"codex exec failed: {result.stderr.strip()}")
    state.codex_last_message = str(last_message)


def run_infer_format(state: PipelineState) -> None:
    prompt = f"""
Ты Codex-агент. Обнови документацию по RAC-команде.

Контекст:
- Команда RAC: {state.command}
- Док-файл: {state.doc_path}
- Артефакты:
  - {state.response_hex}
  - {state.c2s_decode}
  - {state.s2c_decode}
  - {ROOT / state.artifacts_dir / f"{state.capture_name}_rac.out"}

Требования:
1) В "Поля ответа (из rac)" и "Поля запроса (из rac)" у всех полей должен быть заполнен "Order In Capture".
2) В "Record Layout" не должно быть gap-полей. Заполни все поля и их порядок.
3) Сохрани стиль и структуру документа, не удаляй предыдущие секции.
4) Если не хватает данных, добавь "Open Questions" и "Gap Analysis", но все равно укажи порядок и поля без gap.

Обнови файл {state.doc_path}.
""".strip()
    codex_exec(prompt, state)


def section_table_rows(text: str, header: str) -> List[List[str]]:
    pattern = re.compile(rf"{re.escape(header)}.*?\n(.*?)(\n### |\n## |\Z)", re.S)
    match = pattern.search(text)
    if not match:
        return []
    block = match.group(1)
    rows = []
    for line in block.splitlines():
        if "|" not in line:
            continue
        if re.match(r"\s*\|?\s*-+", line):
            continue
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) >= 4:
            rows.append(cells)
    return rows


def record_layout_rows(text: str) -> List[List[str]]:
    pattern = re.compile(r"### Record Layout.*?\n(.*?)(\n### |\n## |\Z)", re.S)
    match = pattern.search(text)
    if not match:
        return []
    block = match.group(1)
    rows = []
    for line in block.splitlines():
        if "|" not in line:
            continue
        if re.match(r"\s*\|?\s*-+", line):
            continue
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) >= 4:
            rows.append(cells)
    return rows


def parse_command_tokens(command: str) -> Tuple[str, str]:
    tokens = command.split()
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


def score_doc(state: PipelineState) -> Tuple[float, List[str]]:
    doc_path = ROOT / state.doc_path
    text = doc_path.read_text(errors="replace")
    reasons = []

    for header in ["### Поля ответа (из `rac`)", "### Поля ответа (из rac)"]:
        rows = section_table_rows(text, header)
        if rows:
            for row in rows:
                order = row[-1]
                if not order or order in {"-", "—", "?"}:
                    reasons.append("Missing Order In Capture in response fields table.")
                    break
            break
    for header in ["### Поля запроса (из `rac`)", "### Поля запроса (из rac)"]:
        rows = section_table_rows(text, header)
        if rows:
            for row in rows:
                order = row[-1]
                if not order or order in {"-", "—", "?"}:
                    reasons.append("Missing Order In Capture in request fields table.")
                    break
            break

    layout_rows = record_layout_rows(text)
    for row in layout_rows:
        field = row[2].lower() if len(row) > 2 else ""
        notes = row[-1].lower() if row else ""
        if "gap" in field or "gap" in notes or "unknown" in field or "unknown" in notes:
            reasons.append("Record Layout contains gap/unknown fields.")
            break

    score = 1.0 if not reasons else 0.0
    return score, reasons


def user_query(state: PipelineState) -> None:
    if not state.score_reasons:
        return
    print("Нужны уточнения для продолжения:")
    for idx, reason in enumerate(state.score_reasons, 1):
        print(f"{idx}. {reason}")
    print("Ответь коротко (можно по пунктам).")
    answer = sys.stdin.readline().strip()
    if answer:
        state.user_answers["general"] = answer


def review_and_fix(state: PipelineState) -> None:
    prompt = f"""
Ты Codex-агент. Исправь документацию, чтобы пройти критерии качества.

Критерии:
- Все поля в "Поля ответа/запроса" имеют заполненный порядок следования.
- В "Record Layout" отсутствуют gap/unknown поля.

Причины провала:
{chr(10).join(state.score_reasons) if state.score_reasons else "-"}

Ответы пользователя:
{state.user_answers.get("general", "-")}

Исправь файл: {state.doc_path}
""".strip()
    codex_exec(prompt, state)


def run_pipeline(args: argparse.Namespace) -> None:
    env_file = Path(args.env_file)
    env_values = load_env_file(env_file)
    command = substitute_command(args.command, env_values)
    state = PipelineState(
        command=command,
        capture_name=args.capture_name,
        doc_path=args.doc,
        artifacts_dir=args.artifacts_dir,
    )
    state_path = ROOT / state.artifacts_dir / f"{state.capture_name}_pipeline_state.json"
    runner_env = os.environ.copy()
    runner_env.update(env_values)
    rac_path = env_values.get("rac_path")
    if rac_path and "RAC_BIN" not in runner_env:
        runner_env["RAC_BIN"] = str(Path(rac_path) / "rac")
    if "TARGET_ADDR" not in runner_env and "endpoint" in env_values:
        runner_env["TARGET_ADDR"] = env_values["endpoint"]

    if args.build:
        build = run_cmd(["cargo", "build", "--release"], ROOT, env=runner_env)
        if build.returncode != 0:
            raise RuntimeError(build.stderr.strip())

    if args.capture:
        run_capture(state, runner_env)
        write_state(state_path, state)

    if args.decode:
        run_decode(state)
        write_state(state_path, state)

    if args.extract:
        extract_artifacts(state)
        write_state(state_path, state)

    if args.infer:
        run_infer_format(state)
        write_state(state_path, state)

    if args.score:
        score, reasons = score_doc(state)
        state.score = score
        state.score_reasons = reasons
        state.needs_user = score < 1.0
        write_state(state_path, state)

    if state.needs_user and args.ask_user:
        user_query(state)
        write_state(state_path, state)

    if state.needs_user and args.fix:
        review_and_fix(state)
        score, reasons = score_doc(state)
        state.score = score
        state.score_reasons = reasons
        state.needs_user = score < 1.0
        write_state(state_path, state)

    if args.update_registry:
        mode, command = parse_command_tokens(state.command)
        update_cmd = [
            sys.executable,
            str(ROOT / "scripts" / "update_modes_registry.py"),
            "--mode",
            mode,
            "--command",
            command,
            "--analyzed",
            "yes",
        ]
        update_run = run_cmd(update_cmd, ROOT)
        if update_run.returncode != 0:
            raise RuntimeError(update_run.stderr.strip() or update_run.stdout.strip())

    if state.needs_user:
        print("Pipeline finished with unresolved issues. See state file for details.")
    else:
        print("Pipeline finished successfully.")


def main() -> None:
    parser = argparse.ArgumentParser(description="RAC analysis pipeline orchestrator (local Codex).")
    parser.add_argument("--command", required=True, help="RAC command arguments, e.g. 'server list --cluster <id>'")
    parser.add_argument("--capture-name", required=True, help="Capture name label.")
    parser.add_argument("--doc", required=True, help="Doc file to update, e.g. docs/rac/messages/rac_message_formats_server.md")
    parser.add_argument("--artifacts-dir", default="artifacts", help="Artifacts directory.")
    parser.add_argument(
        "--env-file",
        default="docs/rac/.private/rac_env.txt",
        help="Env file with RAC constants (key=value).",
    )
    parser.add_argument("--build", action="store_true", help="Build release binaries before capture.")
    parser.add_argument("--capture", action="store_true", help="Run capture step.")
    parser.add_argument("--decode", action="store_true", help="Run decode step.")
    parser.add_argument("--extract", action="store_true", help="Extract artifacts step.")
    parser.add_argument("--infer", action="store_true", help="Run Codex inference to update docs.")
    parser.add_argument("--score", action="store_true", help="Score doc quality.")
    parser.add_argument("--ask-user", action="store_true", help="Ask user questions if score fails.")
    parser.add_argument("--fix", action="store_true", help="Run Codex review/fix if score fails.")
    parser.add_argument("--update-registry", action="store_true", help="Update docs/rac/modes/rac_modes_registry.md.")
    parser.add_argument(
        "--all",
        action="store_true",
        help="Run capture, decode, extract, infer, score, ask-user, fix in order.",
    )
    args = parser.parse_args()

    if args.all:
        args.capture = True
        args.decode = True
        args.extract = True
        args.infer = True
        args.score = True
        args.ask_user = True
        args.fix = True
        args.update_registry = True

    run_pipeline(args)


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
import argparse
import json
import os
import re
import subprocess
import sys
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Iterable, List, Optional


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_CODEX_BIN = os.environ.get("CODEX_BIN", "codex")
DEFAULT_CODEX_FLAGS = os.environ.get("CODEX_FLAGS", "--full-auto")


@dataclass
class RegistryEntry:
    mode: str
    description_file: str
    message_formats_file: str
    command: str
    analyzed: str
    implemented: str
    notes: str

    def key(self) -> str:
        return f"{self.mode} {self.command}".strip()


def split_row(line: str) -> List[str]:
    return [cell.strip() for cell in line.strip().strip("|").split("|")]


def normalize_mode(cell: str) -> str:
    return cell.strip().strip("`")


def parse_registry(path: Path) -> List[RegistryEntry]:
    lines = path.read_text().splitlines()
    entries: List[RegistryEntry] = []
    in_table = False
    current_mode = ""

    for line in lines:
        if line.strip().startswith("| Mode") and "Command" in line:
            in_table = True
            continue
        if in_table and line.strip().startswith("|---"):
            continue
        if not in_table:
            continue
        if not line.strip().startswith("|"):
            break
        cells = split_row(line)
        if len(cells) < 7:
            continue
        if cells[0]:
            current_mode = normalize_mode(cells[0])
        if not current_mode:
            continue
        entries.append(
            RegistryEntry(
                mode=current_mode,
                description_file=cells[1],
                message_formats_file=cells[2],
                command=cells[3].strip("`"),
                analyzed=cells[4],
                implemented=cells[5],
                notes=cells[6],
            )
        )
    return entries


def log_step(message: str) -> None:
    ts = datetime.now().isoformat(timespec="seconds")
    print(f"[{ts}] {message}")


def find_entry(entries: Iterable[RegistryEntry], mode: str, command: str) -> Optional[RegistryEntry]:
    for entry in entries:
        if entry.mode == mode and entry.command == command:
            return entry
    return None


def select_entries(entries: Iterable[RegistryEntry], include_unanalyzed: bool) -> List[RegistryEntry]:
    out = []
    for entry in entries:
        if entry.command in {"", "-"}:
            continue
        if not include_unanalyzed and entry.analyzed.strip().lower() != "yes":
            continue
        implemented = entry.implemented.strip().lower()
        if implemented in {"yes"}:
            continue
        out.append(entry)
    return out


def write_plan_md(entries: List[RegistryEntry], path: Path) -> None:
    lines = []
    lines.append("# RAC Implementation Plan")
    lines.append("")
    lines.append("| # | Mode | Command | Description | Message formats | Analyzed | Implemented | Notes |")
    lines.append("|---|------|---------|-------------|-----------------|----------|-------------|-------|")
    for idx, entry in enumerate(entries, 1):
        lines.append(
            "| {idx} | `{mode}` | `{command}` | `{desc}` | `{msg}` | {analyzed} | {impl} | {notes} |".format(
                idx=idx,
                mode=entry.mode,
                command=entry.command,
                desc=entry.description_file or "-",
                msg=entry.message_formats_file or "-",
                analyzed=entry.analyzed or "-",
                impl=entry.implemented or "-",
                notes=entry.notes or "-",
            )
        )
    path.write_text("\n".join(lines) + "\n")


def write_plan_json(entries: List[RegistryEntry], path: Path) -> None:
    payload = [
        {
            "mode": e.mode,
            "command": e.command,
            "description_file": e.description_file,
            "message_formats_file": e.message_formats_file,
            "analyzed": e.analyzed,
            "implemented": e.implemented,
            "notes": e.notes,
        }
        for e in entries
    ]
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n")


def git_diff_files() -> List[Path]:
    result = subprocess.run(
        ["git", "diff", "--name-only"],
        cwd=str(ROOT),
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(result.stderr.strip() or result.stdout.strip())
    files = []
    for line in result.stdout.splitlines():
        line = line.strip()
        if not line:
            continue
        files.append(ROOT / line)
    return files


def git_diff_text(path: Path) -> str:
    result = subprocess.run(
        ["git", "diff", "--", str(path)],
        cwd=str(ROOT),
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(result.stderr.strip() or result.stdout.strip())
    return result.stdout


def review_file(path: Path) -> List[str]:
    text = path.read_text(errors="replace")
    issues: List[str] = []

    if re.search(r"\bscan_[a-zA-Z0-9_]*\b", text):
        issues.append("uses scan_* helpers (semantic scanning)")
    if re.search(r"\.seek\(", text):
        issues.append("uses RecordCursor.seek()")
    if re.search(r"\.skip\(", text):
        issues.append("uses RecordCursor.skip()")
    if re.search(r"\boffset\b", text):
        issues.append("uses offset variables")
    if re.search(r"\bscan_len_prefixed_strings\b", text):
        issues.append("uses scan_len_prefixed_strings()")
    if re.search(r"\bscan_uuid_bytes\b", text):
        issues.append("uses scan_uuid_bytes()")
    if re.search(r"\bscan_prefixed_uuids\b", text):
        issues.append("uses scan_prefixed_uuids()")
    if has_if_in_tests(text):
        issues.append("tests contain if statements (avoid conditional assertions)")
    if needs_record_cursor(text):
        issues.append("parsing should use RecordCursor (missing usage)")
    return issues


def has_if_in_tests(text: str) -> bool:
    # Heuristic: detect `if` inside #[cfg(test)] mod tests { ... } blocks.
    test_blocks = re.findall(r"#\s*\[\s*cfg\s*\(\s*test\s*\)\s*]\s*mod\s+tests\s*{(.*?)}\s*$",
                             text, re.S | re.M)
    for block in test_blocks:
        if re.search(r"\bif\b", block):
            return True
    return False


def needs_record_cursor(text: str) -> bool:
    if "RecordCursor" in text:
        return False
    if re.search(r"\bparse_[A-Za-z0-9_]+\s*\(", text):
        return True
    return False


def codex_exec(prompt: str, label: str) -> Path:
    artifacts = ROOT / "artifacts"
    artifacts.mkdir(parents=True, exist_ok=True)
    last_message = artifacts / f"{label}_codex_last.md"
    env = os.environ.copy()
    env.setdefault("HTTP_PROXY", "http://127.0.0.1:25345")
    env.setdefault("HTTPS_PROXY", "http://127.0.0.1:25345")
    cmd = [
        DEFAULT_CODEX_BIN,
        "exec",
        "-",
        "--cd",
        str(ROOT),
        "--output-last-message",
        str(last_message),
    ] + DEFAULT_CODEX_FLAGS.split()
    result = subprocess.run(
        cmd,
        cwd=str(ROOT),
        capture_output=True,
        text=True,
        input=prompt,
        env=env,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(result.stderr.strip() or result.stdout.strip())
    return last_message


def run_review(paths: List[Path], fail_on_warn: bool) -> None:
    findings = []
    global_issues: List[str] = []

    mod_rs = ROOT / "apps" / "rac_protocol" / "src" / "commands" / "mod.rs"
    if mod_rs in paths:
        diff = git_diff_text(mod_rs)
        if re.search(r"^\+\s*pub fn\b", diff, re.M):
            global_issues.append(
                "commands/mod.rs adds pub fn; implement per-mode commands in their own modules"
            )
    for path in paths:
        if not path.exists() or path.suffix != ".rs":
            continue
        issues = review_file(path)
        if issues:
            findings.append((path, issues))

    if findings or global_issues:
        print("Review findings:")
        for issue in global_issues:
            print(f"- {issue}")
        for path, issues in findings:
            print(f"- {path}")
            for issue in issues:
                print(f"  - {issue}")
        if fail_on_warn:
            raise SystemExit(1)
    else:
        print("Review OK: no findings.")


def update_registry(mode: str, command: str, implemented: str) -> None:
    cmd = [
        sys.executable,
        str(ROOT / "scripts" / "update_modes_registry.py"),
        "--mode",
        mode,
        "--command",
        command,
        "--implemented",
        implemented,
    ]
    result = subprocess.run(cmd, cwd=str(ROOT), capture_output=True, text=True, check=False)
    if result.returncode != 0:
        raise RuntimeError(result.stderr.strip() or result.stdout.strip())


def git_is_dirty() -> bool:
    result = subprocess.run(
        ["git", "status", "--porcelain"],
        cwd=str(ROOT),
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(result.stderr.strip() or result.stdout.strip())
    return bool(result.stdout.strip())


def git_has_tracked_changes() -> bool:
    result = subprocess.run(
        ["git", "status", "--porcelain"],
        cwd=str(ROOT),
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(result.stderr.strip() or result.stdout.strip())
    for line in result.stdout.splitlines():
        if not line:
            continue
        if not line.startswith("??"):
            return True
    return False


def commit_if_needed(message: str, label: str) -> None:
    if git_has_tracked_changes():
        log_step(f"{label}: commit {message}")
        git_commit(message)
        log_step(f"{label}: commit created")
        return
    if git_is_dirty():
        log_step(f"{label}: skip commit (only untracked files present)")
    else:
        log_step(f"{label}: skip commit (no changes)")


def refresh_plan(registry: str = "docs/rac/modes/rac_modes_registry.md") -> None:
    registry_path = ROOT / registry
    entries = parse_registry(registry_path)
    selected = select_entries(entries, include_unanalyzed=False)
    write_plan_md(selected, ROOT / "artifacts" / "rac_impl_plan.md")
    write_plan_json(selected, ROOT / "artifacts" / "rac_impl_plan.json")


def git_commit(message: str) -> None:
    result = subprocess.run(
        ["git", "commit", "-am", message],
        cwd=str(ROOT),
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(result.stderr.strip() or result.stdout.strip())


def main() -> None:
    parser = argparse.ArgumentParser(description="RAC implementation pipeline helper.")
    sub = parser.add_subparsers(dest="cmd", required=True)

    plan = sub.add_parser("plan", help="Generate implementation plan from registry.")
    plan.add_argument("--registry", default="docs/rac/modes/rac_modes_registry.md")
    plan.add_argument("--out-md", default="artifacts/rac/rac_impl_plan.md")
    plan.add_argument("--out-json", default="artifacts/rac/rac_impl_plan.json")
    plan.add_argument("--include-unanalyzed", action="store_true")

    review = sub.add_parser("review", help="Review code for sequential parsing constraints.")
    review.add_argument("--paths", nargs="*", default=None, help="Files to review; default: git diff.")
    review.add_argument("--fail-on-warn", action="store_true")
    review.add_argument("--codex", action="store_true", help="Run Codex review (and optional fix).")
    review.add_argument("--fix", action="store_true", help="Let Codex fix review findings.")
    review.add_argument("--label", default="codex_review")

    implement = sub.add_parser("implement", help="Run Codex implementation for a command.")
    implement.add_argument("--mode", required=True)
    implement.add_argument("--command", required=True, help="Command words (e.g. 'list' or 'summary list').")
    implement.add_argument("--registry", default="docs/rac/modes/rac_modes_registry.md")
    implement.add_argument("--label", default=None)

    update = sub.add_parser("update-registry", help="Mark a command as implemented in the registry.")
    update.add_argument("--mode", required=True)
    update.add_argument("--command", required=True, help="Command words (e.g. 'list' or 'summary list').")
    update.add_argument("--implemented", default="yes")

    commit = sub.add_parser("commit", help="Create a git commit for current changes.")
    commit.add_argument("--message", required=True)

    run = sub.add_parser("run", help="Full pipeline: implement -> review -> registry -> commit.")
    run.add_argument("--mode", required=True)
    run.add_argument("--command", required=True, help="Command words (e.g. 'list' or 'summary list').")
    run.add_argument("--registry", default="docs/rac/modes/rac_modes_registry.md")
    run.add_argument("--label", default=None)
    run.add_argument("--commit-message", default=None)
    run.add_argument("--codex-review", action="store_true", help="Run Codex review+fix step.")

    run_next = sub.add_parser(
        "run-next",
        help="Run pipeline for first not implemented command from artifacts/rac/rac_impl_plan.json.",
    )
    run_next.add_argument("--plan", default="artifacts/rac/rac_impl_plan.json")
    run_next.add_argument("--registry", default="docs/rac/modes/rac_modes_registry.md")
    run_next.add_argument("--codex-review", action="store_true", help="Run Codex review+fix step.")

    run_all = sub.add_parser(
        "run-all",
        help="Run pipeline sequentially for all not implemented commands from artifacts/rac/rac_impl_plan.json.",
    )
    run_all.add_argument("--plan", default="artifacts/rac/rac_impl_plan.json")
    run_all.add_argument("--registry", default="docs/rac/modes/rac_modes_registry.md")
    run_all.add_argument("--codex-review", action="store_true", help="Run Codex review+fix step.")

    args = parser.parse_args()

    if args.cmd == "plan":
        registry_path = ROOT / args.registry
        entries = parse_registry(registry_path)
        selected = select_entries(entries, include_unanalyzed=args.include_unanalyzed)
        out_md = ROOT / args.out_md
        out_json = ROOT / args.out_json
        write_plan_md(selected, out_md)
        write_plan_json(selected, out_json)
        print(f"Wrote {out_md}")
        print(f"Wrote {out_json}")
        return

    if args.cmd == "review":
        if args.paths:
            paths = [ROOT / p for p in args.paths]
        else:
            paths = git_diff_files()
        log_step("review: pre-check")
        run_review(paths, args.fail_on_warn)
        if args.codex:
            precheck_note = "Pre-check: OK (no findings)."
            if not args.fail_on_warn:
                # Re-run to collect findings without failing.
                findings = []
                for path in paths:
                    if not path.exists() or path.suffix != ".rs":
                        continue
                    issues = review_file(path)
                    if issues:
                        findings.append((path, issues))
                if findings:
                    lines = ["Pre-check findings:"]
                    for path, issues in findings:
                        lines.append(f"- {path}")
                        for issue in issues:
                            lines.append(f"  - {issue}")
                    precheck_note = "\n".join(lines)
            prompt = f"""
Ты Codex-агент. Проведи ревью изменений в репозитории.

Требования:
- Запрещен семантический анализ: нельзя использовать scan_* и эвристические сканы.
- Парсинг должен быть строго последовательным чтением (без offset, seek/skip).
- Для чтения должен использоваться RecordCursor.
- Реализация команд каждого режима должна быть в отдельном модуле `apps/rac_protocol/src/commands/<mode>.rs`.
- В тестах не допускаются if/ветвления. Только четкие проверки результата.

{precheck_note}

Действия:
1) Найди проблемы и опиши их.
2) Если запущено с --fix, исправь код и тесты.
3) Затем повторно проверь, что нарушений нет.
""".strip()
            label = args.label
            log_step("review: codex")
            last = codex_exec(prompt, label)
            log_step(f"review: codex complete ({last})")
        return

    if args.cmd == "implement":
        registry_path = ROOT / args.registry
        entries = parse_registry(registry_path)
        entry = find_entry(entries, args.mode, args.command)
        label = args.label or f"{args.mode}_{args.command.replace(' ', '_')}_impl"
        doc_path = entry.message_formats_file if entry else "-"
        log_step(f"task: implement {args.mode} {args.command}")
        prompt = f"""
Ты Codex-агент. Реализуй RAC-команду в библиотеке и CLI.

Команда: {args.mode} {args.command}
Документация формата сообщений: {doc_path}

Требования реализации:
1) Добавить нужные классы/структуры, encode/decode.
2) Добавить тесты с четкими проверками результата.
3) Добавить вывод в console output (rac_lite).
4) Парсинг строго последовательный (без offset/seek/skip).
5) Запрещен семантический анализ (scan_*).
6) Для чтения должен использоваться RecordCursor.
7) Реализация команд каждого режима должна быть в отдельном модуле `apps/rac_protocol/src/commands/<mode>.rs`.
8) В тестах нельзя использовать if или подгонку под реализацию.

Сделай изменения в коде. Коммит не делай.
""".strip()
        log_step("implement: codex")
        last = codex_exec(prompt, label)
        log_step(f"implement: codex complete ({last})")
        return

    if args.cmd == "update-registry":
        log_step(f"registry: update {args.mode} {args.command}")
        update_registry(args.mode, args.command, args.implemented)
        refresh_plan()
        log_step("registry: updated")
        return

    if args.cmd == "commit":
        log_step(f"commit: {args.message}")
        git_commit(args.message)
        log_step("commit: created")
        return

    if args.cmd == "run":
        registry_path = ROOT / args.registry
        entries = parse_registry(registry_path)
        entry = find_entry(entries, args.mode, args.command)
        label = args.label or f"{args.mode}_{args.command.replace(' ', '_')}"
        doc_path = entry.message_formats_file if entry else "-"
        log_step(f"task: run pipeline for {args.mode} {args.command}")

        implement_prompt = f"""
Ты Codex-агент. Реализуй RAC-команду в библиотеке и CLI.

Команда: {args.mode} {args.command}
Документация формата сообщений: {doc_path}

Требования реализации:
1) Добавить нужные классы/структуры, encode/decode.
2) Добавить тесты с четкими проверками результата.
3) Добавить вывод в console output (rac_lite).
4) Парсинг строго последовательный (без offset/seek/skip).
5) Запрещен семантический анализ (scan_*).
6) Для чтения должен использоваться RecordCursor.
7) Реализация команд каждого режима должна быть в отдельном модуле `apps/rac_protocol/src/commands/<mode>.rs`.
8) В тестах нельзя использовать if или подгонку под реализацию.

Сделай изменения в коде. Коммит не делай.
""".strip()
        log_step("implement: codex")
        codex_exec(implement_prompt, f"{label}_impl")
        log_step("implement: codex complete")

        commit_message = args.commit_message or f"impl: {args.mode} {args.command}"
        commit_if_needed(commit_message, "impl")

        # Pre-check (optional Codex review)
        paths = git_diff_files()
        log_step("review: pre-check")
        run_review(paths, False)
        precheck_note = "Pre-check: OK (no findings)."
        findings = []
        for path in paths:
            if not path.exists() or path.suffix != ".rs":
                continue
            issues = review_file(path)
            if issues:
                findings.append((path, issues))
        if findings:
            lines = ["Pre-check findings:"]
            for path, issues in findings:
                lines.append(f"- {path}")
                for issue in issues:
                    lines.append(f"  - {issue}")
            precheck_note = "\n".join(lines)
        if args.codex_review:
            review_prompt = f"""
Ты Codex-агент. Проведи ревью изменений в репозитории и исправь замечания.

Требования:
- Запрещен семантический анализ: нельзя использовать scan_* и эвристические сканы.
- Парсинг должен быть строго последовательным чтением (без offset, seek/skip).
- Для чтения должен использоваться RecordCursor.
- Реализация команд каждого режима должна быть в отдельном модуле `apps/rac_protocol/src/commands/<mode>.rs`.
- В тестах не допускаются if/ветвления. Только четкие проверки результата.

{precheck_note}

        Действия:
1) Найди проблемы и исправь их.
2) Затем повторно проверь, что нарушений нет.
""".strip()
            log_step("review: codex")
            codex_exec(review_prompt, f"{label}_review")
            log_step("review: codex complete")
        elif precheck_note != "Pre-check: OK (no findings).":
            log_step(precheck_note)

        log_step("registry: update implemented=yes")
        update_registry(args.mode, args.command, "yes")
        refresh_plan()
        log_step("registry: updated")

        commit_message = f"review: {args.mode} {args.command}"
        commit_if_needed(commit_message, "review")
        return

    if args.cmd == "run-next":
        plan_path = ROOT / args.plan
        if not plan_path.exists():
            raise RuntimeError(f"Plan not found: {plan_path}")
        entries = json.loads(plan_path.read_text())
        next_entry = None
        for entry in entries:
            implemented = str(entry.get("implemented", "")).strip().lower()
            if implemented == "yes":
                continue
            next_entry = entry
            break
        if not next_entry:
            print("No not-implemented commands found in plan.")
            return
        mode = next_entry["mode"]
        command = next_entry["command"]
        cmd = [
            sys.executable,
            str(Path(__file__).resolve()),
            "run",
            "--mode",
            mode,
            "--command",
            command,
            "--registry",
            args.registry,
        ]
        if args.codex_review:
            cmd.append("--codex-review")
        result = subprocess.run(cmd, cwd=str(ROOT), check=False)
        if result.returncode != 0:
            raise SystemExit(result.returncode)
        return

    if args.cmd == "run-all":
        plan_path = ROOT / args.plan
        if not plan_path.exists():
            raise RuntimeError(f"Plan not found: {plan_path}")
        entries = json.loads(plan_path.read_text())
        queue = []
        for entry in entries:
            implemented = str(entry.get("implemented", "")).strip().lower()
            if implemented == "yes":
                continue
            queue.append(entry)
        if not queue:
            print("No not-implemented commands found in plan.")
            return
        total = len(queue)
        for idx, entry in enumerate(queue, 1):
            mode = entry["mode"]
            command = entry["command"]
            log_step(f"run-all: start {idx}/{total} {mode} {command}")
            cmd = [
                sys.executable,
                str(Path(__file__).resolve()),
                "run",
                "--mode",
                mode,
                "--command",
                command,
                "--registry",
                args.registry,
            ]
            if args.codex_review:
                cmd.append("--codex-review")
            result = subprocess.run(cmd, cwd=str(ROOT), check=False)
            if result.returncode != 0:
                raise SystemExit(result.returncode)
            log_step(f"run-all: done {idx}/{total} {mode} {command}")
        return


if __name__ == "__main__":
    main()

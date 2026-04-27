#!/usr/bin/env python3

from __future__ import annotations

import re
import shutil
from pathlib import Path


SOURCE_PATTERN = re.compile(r"source:\s+(.+\.proto)")
INCLUDE_PATTERN = re.compile(r'include!\("([^"]+)"\);')
PACKAGE_PATTERN = re.compile(r"^\s*package\s+([A-Za-z0-9_.]+)\s*;")


def _read_source_path(cs_file: Path) -> Path | None:
    with cs_file.open("r", encoding="utf-8") as handle:
        for _ in range(30):
            line = handle.readline()
            if not line:
                break
            match = SOURCE_PATTERN.search(line)
            if match:
                source = match.group(1).strip().lstrip("./")
                return Path(source).parent
    return None


def normalize_csharp_tree(workspace_root: Path) -> tuple[int, int]:
    csharp_root = workspace_root / "csharp" / "proto"
    if not csharp_root.exists():
        return 0, 0

    rebuilt_files = 0
    skipped_files = 0
    cs_files = sorted(csharp_root.rglob("*.cs"))
    tmp_root = workspace_root / "csharp" / ".proto_source_relative_tmp"

    if tmp_root.exists():
        shutil.rmtree(tmp_root)
    tmp_root.mkdir(parents=True, exist_ok=True)

    for cs_file in cs_files:
        source_dir = _read_source_path(cs_file)
        if source_dir is None:
            skipped_files += 1
            continue

        target_dir = tmp_root / source_dir
        target_dir.mkdir(parents=True, exist_ok=True)
        target_file = target_dir / cs_file.name

        if target_file.exists():
            if cs_file.read_bytes() != target_file.read_bytes():
                raise RuntimeError(
                    f"Conflicting generated C# file for {target_file}: "
                    f"{cs_file} and {target_file} differ."
                )
        else:
            shutil.copy2(cs_file, target_file)

        rebuilt_files += 1

    shutil.rmtree(csharp_root)
    tmp_root.rename(csharp_root)

    legacy_multisig = csharp_root / "cosmos" / "crypto" / "multisig" / "Multisig.cs"
    canonical_multisig = (
        csharp_root / "cosmos" / "crypto" / "multisig" / "v1beta1" / "Multisig.cs"
    )
    if canonical_multisig.exists() and not legacy_multisig.exists():
        legacy_multisig.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(canonical_multisig, legacy_multisig)
        rebuilt_files += 1

    return rebuilt_files, skipped_files


def _render_exchange_module(package: str, filename: str) -> list[str]:
    segments = package.split(".")
    lines: list[str] = []
    indent = ""
    for index, segment in enumerate(segments):
        is_last = index == len(segments) - 1
        if is_last:
            lines.append(f"{indent}// @@protoc_insertion_point(attribute:{package})")
        lines.append(f"{indent}pub mod {segment} {{")
        indent += "    "
        if is_last:
            lines.append(f'{indent}include!("{filename}");')
            lines.append(f"{indent}// @@protoc_insertion_point({package})")
    for index in range(len(segments)):
        indent = "    " * (len(segments) - index - 1)
        lines.append(f"{indent}}}")
    return lines


def normalize_rust_tree(workspace_root: Path) -> tuple[int, int]:
    rust_root = workspace_root / "rust" / "proto"
    if not rust_root.exists():
        return 0, 0

    package_to_dirs: dict[str, set[str]] = {}
    proto_root = workspace_root / "proto"
    if proto_root.exists():
        for proto_file in sorted(proto_root.rglob("*.proto")):
            package_name = None
            with proto_file.open("r", encoding="utf-8") as handle:
                for line in handle:
                    match = PACKAGE_PATTERN.match(line)
                    if match:
                        package_name = match.group(1)
                        break
            if package_name is None:
                continue
            package_to_dirs.setdefault(package_name, set()).add(
                proto_file.parent.relative_to(proto_root).as_posix()
            )

    moved_files = 0
    include_rewrites: dict[str, str] = {}
    rust_files = sorted(
        path
        for path in rust_root.rglob("*.rs")
        if path.name != "mod.rs"
    )
    for rust_file in rust_files:
        if rust_file.name.endswith(".tonic.rs"):
            package_name = rust_file.name[: -len(".tonic.rs")]
        else:
            package_name = rust_file.name[: -len(".rs")]

        candidate_dirs = package_to_dirs.get(package_name)
        if not candidate_dirs or len(candidate_dirs) != 1:
            continue

        target_dir = rust_root / next(iter(candidate_dirs))
        target_dir.mkdir(parents=True, exist_ok=True)
        target_file = target_dir / rust_file.name
        if rust_file == target_file:
            continue
        if target_file.exists() and rust_file.read_bytes() != target_file.read_bytes():
            raise RuntimeError(
                f"Conflicting generated Rust file for {target_file}: "
                f"{rust_file} and {target_file} differ."
            )

        source_rel = rust_file.relative_to(rust_root).as_posix()
        target_rel = target_file.relative_to(rust_root).as_posix()
        include_rewrites[source_rel] = target_rel

        if not target_file.exists():
            shutil.move(str(rust_file), str(target_file))
        else:
            rust_file.unlink()
        moved_files += 1

    mod_file = rust_root / "mod.rs"
    if mod_file.exists() and include_rewrites:
        mod_content = mod_file.read_text(encoding="utf-8")
        for source_rel, target_rel in include_rewrites.items():
            mod_content = mod_content.replace(
                f'include!("{source_rel}");',
                f'include!("{target_rel}");',
            )
        mod_file.write_text(mod_content, encoding="utf-8")

    exchange_packages: dict[str, str] = {}
    for rust_file in sorted((rust_root / "exchange").glob("*.rs")):
        if rust_file.name == "mod.rs" or rust_file.name.endswith(".tonic.rs"):
            continue
        exchange_packages[rust_file.stem] = rust_file.name
    if exchange_packages:
        exchange_mod = rust_root / "exchange" / "mod.rs"
        exchange_mod.parent.mkdir(parents=True, exist_ok=True)
        lines = ["// @generated"]
        for package in sorted(exchange_packages):
            lines.extend(_render_exchange_module(package, exchange_packages[package]))
        exchange_mod.write_text("\n".join(lines) + "\n", encoding="utf-8")

    canonical_multisig = (
        rust_root
        / "cosmos"
        / "crypto"
        / "multisig"
        / "v1beta1"
        / "cosmos.crypto.multisig.v1beta1.rs"
    )
    legacy_multisig = (
        rust_root / "cosmos" / "crypto" / "multisig" / "cosmos.crypto.multisig.v1beta1.rs"
    )
    if canonical_multisig.exists() and not legacy_multisig.exists():
        legacy_multisig.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(canonical_multisig, legacy_multisig)
        moved_files += 1

    touched_files = 0
    renamed_symbols = 0

    for mod_file in sorted(rust_root.rglob("mod.rs")):
        content = mod_file.read_text(encoding="utf-8")
        for include_path in INCLUDE_PATTERN.findall(content):
            target_file = mod_file.parent / include_path
            if target_file.exists():
                continue
            target_file.parent.mkdir(parents=True, exist_ok=True)
            target_file.touch()
            touched_files += 1

    staking_file = (
        rust_root
        / "cosmos"
        / "staking"
        / "v1beta1"
        / "cosmos.staking.v1beta1.rs"
    )
    if staking_file.exists():
        staking_content = staking_file.read_text(encoding="utf-8")
        updated_content = (
            staking_content.replace("pub enum Validators", "pub enum EnumValidators")
            .replace(
                "stake_authorization::Validators",
                "stake_authorization::EnumValidators",
            )
        )
        if updated_content != staking_content:
            staking_file.write_text(updated_content, encoding="utf-8")
            renamed_symbols += 1

    return touched_files + moved_files, renamed_symbols


def main() -> None:
    workspace_root = Path(__file__).resolve().parent.parent

    csharp_moved, csharp_skipped = normalize_csharp_tree(workspace_root)
    rust_touched, rust_renamed = normalize_rust_tree(workspace_root)

    print(
        "normalize-generated: "
        f"csharp_moved={csharp_moved} "
        f"csharp_skipped={csharp_skipped} "
        f"rust_touched={rust_touched} "
        f"rust_renamed={rust_renamed}"
    )


if __name__ == "__main__":
    main()

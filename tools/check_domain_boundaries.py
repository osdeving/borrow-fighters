#!/usr/bin/env python3
"""Check the local Rust domain contract without compiling either game.

This is a small source scanner, not a Rust compiler: it expands use trees and
resolves crate/self/super paths and import aliases, ignoring comments and string
contents. Cargo's separate feature builds remain the check for macro expansion
and type resolution. Keep the rules specific to the package's two domains and
their default composition root; presentation is never shared core.

Requires Python 3.11+ for stdlib tomllib. In the development environment run:
python3.13 tools/check_domain_boundaries.py
"""

from __future__ import annotations

import argparse
from dataclasses import dataclass
from pathlib import Path
import posixpath
import re
import sys
try:
    import tomllib
except ModuleNotFoundError:
    raise SystemExit("Python 3.11+ is required; locally run python3.13 tools/check_domain_boundaries.py") from None

CORE = {"math", "runtime_paths"}
DOMAINS = {"fighting", "adventure"}
PRESENTATION = "src/presentation.rs"
STORY_BINARY = "src/bin/borrow-story.rs"
ACTOR_LAB_BINARY = "src/bin/borrow-actor-lab.rs"
PRESENTATION_APIS = {"app", "cli", "config"}
IDENT = re.compile(r"(?:r#)?[A-Za-z_][A-Za-z_0-9]*")


@dataclass(frozen=True)
class Token:
    value: str
    kind: str
    line: int


@dataclass(frozen=True)
class Import:
    parts: tuple[str, ...]
    alias: str | None
    line: int
    context: tuple[str, ...]


def tokens(source: str) -> list[Token]:
    """Tokenize paths/attributes; discard nested comments and character literals."""
    result = []
    index = 0
    line = 1
    while index < len(source):
        start = index
        if source[index].isspace():
            index += 1
        elif source.startswith("//", index):
            end = source.find("\n", index)
            index = len(source) if end < 0 else end
        elif source.startswith("/*", index):
            depth = 1
            index += 2
            while index < len(source) and depth:
                if source.startswith("/*", index):
                    depth += 1
                    index += 2
                elif source.startswith("*/", index):
                    depth -= 1
                    index += 2
                else:
                    index += 1
        elif match := re.match(r'(?:br|cr|r)(#*)"', source[index:]):
            content = index + match.end()
            suffix = '"' + match.group(1)
            end = source.find(suffix, content)
            end = len(source) if end < 0 else end
            result.append(Token(source[content:end], "string", line))
            index = min(len(source), end + len(suffix))
        elif source[index] == '"' or source[index:index + 2] in {'b"', 'c"'}:
            index += 1 if source[index] == '"' else 2
            content = index
            while index < len(source) and source[index] != '"':
                index += 2 if source[index] == "\\" else 1
            result.append(Token(source[content:index], "string", line))
            index = min(len(source), index + 1)
        elif match := re.match(r"(?:b)?'(?:\\(?:u\{[0-9A-Fa-f_]+\}|x[0-9A-Fa-f]{2}|.)|[^'\\\n])'", source[index:]):
            index += match.end()
        elif match := IDENT.match(source, index):
            result.append(Token(match.group().removeprefix("r#"), "ident", line))
            index = match.end()
        elif source.startswith("::", index):
            result.append(Token("::", "punct", line))
            index += 2
        else:
            result.append(Token(source[index], "punct", line))
            index += 1
        line += source[start:index].count("\n")

    # #[doc = "..."] is documentation, including when it mentions asset paths.
    filtered = []
    index = 0
    while index < len(result):
        if [t.value for t in result[index:index + 3]] == ["#", "[", "doc"]:
            depth = 1
            index += 3
            while index < len(result) and depth:
                if result[index].value == "[":
                    depth += 1
                elif result[index].value == "]":
                    depth -= 1
                index += 1
        else:
            filtered.append(result[index])
            index += 1
    return filtered


def module_context(path: Path) -> tuple[str, ...]:
    if path.as_posix() == "src/lib.rs":
        return ()
    parts = list(path.with_suffix("").parts)
    if parts[0] != "src" or path.as_posix() == "src/main.rs" or parts[1] == "bin":
        return ("<local-target>",)
    parts = parts[1:]
    if parts[-1] == "mod":
        parts.pop()
    return tuple(parts)


def contexts(items: list[Token], initial: tuple[str, ...]) -> list[tuple[str, ...]]:
    """Track inline modules so test-module super imports resolve correctly."""
    result = []
    current = initial
    stack = []
    for index, token in enumerate(items):
        result.append(current)
        if token.value == "{":
            stack.append(current)
            if index >= 2 and items[index - 2].value == "mod" and items[index - 1].kind == "ident":
                current += (items[index - 1].value,)
        elif token.value == "}" and stack:
            current = stack.pop()
    return result


def use_tree(items: list[Token], context: tuple[str, ...], prefix: tuple[str, ...] = ()) -> list[Import]:
    """Expand nested use trees, preserving aliases and self/glob leaves."""
    result = []
    index = 0
    while index < len(items):
        parts = list(prefix)
        alias = None
        line = items[index].line
        while index < len(items) and items[index].value not in {",", "{"}:
            value = items[index].value
            if value == "as":
                index += 1
                if index < len(items):
                    alias = items[index].value
                    index += 1
                break
            if items[index].kind == "ident" or value == "*":
                parts.append(value)
            index += 1
        if index < len(items) and items[index].value == "{":
            start = index + 1
            depth = 1
            index += 1
            while index < len(items) and depth:
                if items[index].value == "{":
                    depth += 1
                elif items[index].value == "}":
                    depth -= 1
                index += 1
            result.extend(use_tree(items[start:index - 1], context, tuple(parts)))
        elif parts:
            if parts[-1] == "self" and len(parts) > 1:
                parts.pop()
            result.append(Import(tuple(parts), alias, line, context))
        if index < len(items) and items[index].value == ",":
            index += 1
    return result


def imports(items: list[Token], scope: list[tuple[str, ...]]) -> tuple[list[Import], set[int]]:
    result = []
    covered = set()
    index = 0
    while index < len(items):
        if items[index].kind == "ident" and items[index].value == "use":
            start = index
            while index < len(items) and items[index].value != ";":
                index += 1
            result.extend(use_tree(items[start + 1:index], scope[start]))
            covered.update(range(start, min(len(items), index + 1)))
        index += 1
    return result, covered


def resolve(parts: tuple[str, ...], context: tuple[str, ...], aliases: dict, target_root: tuple[str, ...] = ()) -> tuple[str, ...] | None:
    if not parts:
        return None
    first, *tail = parts
    if first == "borrow_fighters":
        return tuple(tail)
    if first == "crate":
        return target_root + tuple(tail)
    if first == "self":
        return context + tuple(tail)
    if first == "super":
        result = list(context)
        for part in parts:
            if part == "super":
                if not result:
                    return ("<escaped-crate>",)
                result.pop()
            elif part != "self":
                result.append(part)
        return tuple(result)
    for length in range(len(context), -1, -1):
        if target := aliases.get((context[:length], first)):
            return target + tuple(tail)
    return None  # external dependency or a local unqualified name


def source_paths(source: str, path: Path) -> tuple[list[tuple[tuple[str, ...], int, tuple[str, ...]]], list[Token]]:
    items = tokens(source)
    scope = contexts(items, module_context(path))
    target_root = ("<local-target>",) if module_context(path) == ("<local-target>",) else ()
    uses, covered = imports(items, scope)
    aliases = {}
    # Resolve aliases independent of declaration order, including aliases of aliases.
    for _ in range(len(uses) + 1):
        changed = False
        for entry in uses:
            resolved = resolve(entry.parts, entry.context, aliases, target_root)
            if resolved is not None and resolved:
                name = entry.alias or entry.parts[-1]
                if name != "*" and aliases.get((entry.context, name)) != resolved:
                    aliases[(entry.context, name)] = resolved
                    changed = True
        if not changed:
            break
    paths = []
    for entry in uses:
        resolved = resolve(entry.parts, entry.context, aliases, target_root)
        if resolved is not None:
            paths.append((resolved, entry.line, entry.context))
    for index, token in enumerate(items):
        if index in covered or token.kind != "ident":
            continue
        if index and items[index - 1].value == "::":
            continue
        if index + 1 >= len(items) or items[index + 1].value != "::":
            continue
        parts = [token.value]
        end = index + 1
        while end + 1 < len(items) and items[end].value == "::" and items[end + 1].kind == "ident":
            parts.append(items[end + 1].value)
            end += 2
        resolved = resolve(tuple(parts), scope[index], aliases, target_root)
        if resolved is not None:
            paths.append((resolved, token.line, scope[index]))
    return paths, items


def domain_for(path: Path) -> str:
    if path.as_posix() == "src/lib.rs":
        return "library"
    if path.as_posix() == PRESENTATION or path.parts[:2] == ("src", "presentation"):
        return "presentation"
    if path.as_posix() == STORY_BINARY:
        return "presentation entrypoint"
    if path.as_posix() == ACTOR_LAB_BINARY:
        return "adventure"
    if path.parts[0] == "src":
        top = module_context(path)
        if top and top[0] in CORE:
            return "core"
        if top and top[0] == "adventure":
            return "adventure"
    if "adventure" in re.split(r"[_-]", path.stem):
        return "adventure"
    return "fighting"


def check_source(source: str, path: Path, domain: str) -> list[str]:
    errors = []
    paths, items = source_paths(source, path)
    def scoped_domain(context):
        if domain != "library":
            return domain
        if not context or context[0] in CORE:
            return "core"
        if context[0] == "presentation":
            return "presentation"
        return "adventure" if context[0] == "adventure" else "fighting"

    for parts, line, context in paths:
        owner = scoped_domain(context)
        top = parts[0] if parts else "<crate-root>"
        allowed = top in CORE or top == "<local-target>" or (owner == "adventure" and top == "adventure")
        if owner == "fighting":
            allowed = top not in {"adventure", "presentation", "*", "<crate-root>", "<escaped-crate>"}
        elif owner == "presentation":
            allowed = top in CORE | PRESENTATION_APIS | {"presentation"} or parts[:2] == ("adventure", "app")
        elif owner == "presentation entrypoint":
            allowed = top in CORE | {"presentation", "<local-target>"}
        if not allowed:
            errors.append(f"{path}:{line}: {owner} cannot depend on {'::'.join(parts) or 'the entire crate'}")
    scope = contexts(items, module_context(path))
    for index, (token, context) in enumerate(zip(items, scope)):
        if scoped_domain(context) not in {"presentation", "presentation entrypoint"}:
            continue
        following = [item.value for item in items[index:index + 3]]
        external_module = token.value == "mod" and len(following) == 3 and following[2] == ";"
        if external_module or following == ["#", "[", "path"] or following[:2] == ["include", "!"]:
            errors.append(f"{path}:{token.line}: presentation must keep composition explicit in {PRESENTATION}; no external modules or include!")
    for token, context in zip(items, scope):
        owner = scoped_domain(context)
        if token.kind != "string" or owner not in {"adventure", "presentation", "presentation entrypoint"}:
            continue
        literal = token.value.replace("\\", "/")
        match = re.search(r"(?:^|/)assets(?:/|$)", literal)
        if match:
            asset = posixpath.normpath("assets/" + literal[match.end():])
            if owner != "adventure":
                errors.append(f"{path}:{token.line}: {owner} must leave asset loading to each app: {token.value}")
            elif asset != "assets/adventure" and not asset.startswith("assets/adventure/"):
                errors.append(f"{path}:{token.line}: adventure asset must be under assets/adventure: {token.value}")
    return list(dict.fromkeys(errors))


def check_lib(source: str) -> list[str]:
    items = tokens(source)
    errors = []
    attributes = []
    declared = set()
    index = 0
    body_depth = 0
    while index < len(items):
        if items[index].value == "{":
            body_depth += 1
        elif items[index].value == "}":
            body_depth -= 1
        if body_depth:
            index += 1
            continue
        if [t.value for t in items[index:index + 2]] == ["#", "["]:
            start = index + 2
            depth = 1
            index += 2
            while index < len(items) and depth:
                depth += (items[index].value == "[") - (items[index].value == "]")
                index += 1
            attributes.append([t.value for t in items[start:index - 1]])
            continue
        if items[index].value == "mod" and index + 1 < len(items):
            name = items[index + 1].value
            declared.add(name)
            expected = None if name in CORE else "adventure" if name == "adventure" else "fighting"
            cfgs = [attribute for attribute in attributes if attribute and attribute[0] in {"cfg", "cfg_attr"}]
            needed = ["cfg", "(", "feature", "=", expected, ")"]
            if name == "presentation":
                # Accept either ordering, but no any/test/cfg_attr escape hatch.
                gates = [
                    ["cfg", "(", "all", "(", "feature", "=", first, ",", "feature", "=", second, ")", ")"]
                    for first, second in (("adventure", "fighting"), ("fighting", "adventure"))
                ]
                if cfgs not in [[gate] for gate in gates]:
                    errors.append(f'src/lib.rs:{items[index].line}: module presentation must have exactly #[cfg(all(feature = "adventure", feature = "fighting"))]')
            elif (expected is None and cfgs) or (expected is not None and cfgs != [needed]):
                errors.append(f"src/lib.rs:{items[index].line}: module {name} must {'be ungated core' if expected is None else 'have exactly #[cfg(feature = ' + repr(expected) + ')]'}")
            attributes = []
        elif items[index].value in {";", "}"}:
            attributes = []
        index += 1
    for name in CORE | {"adventure", "presentation"}:
        if name not in declared:
            errors.append(f"src/lib.rs: missing domain/core module {name}")
    return errors


def check_repository(root: Path) -> list[str]:
    errors = []
    try:
        manifest = tomllib.loads((root / "Cargo.toml").read_text())
    except (OSError, tomllib.TOMLDecodeError) as error:
        return [f"Cargo.toml: {error}"]
    package = manifest.get("package", {})
    if package.get("default-run") != "borrow-story":
        errors.append('Cargo.toml: package.default-run must be "borrow-story"')
    for discovery in ("autotests", "autoexamples"):
        if package.get(discovery) is not False:
            errors.append(f"Cargo.toml: package.{discovery} must be false; explicitly register every target")
    features = manifest.get("features", {})
    if not DOMAINS.issubset(features):
        errors.append("Cargo.toml: fighting and adventure features must exist")
    if sorted(features.get("default", [])) != sorted(DOMAINS):
        errors.append("Cargo.toml: default must enable exactly fighting and adventure")
    for domain in DOMAINS:
        enabled = set()
        pending = [domain]
        while pending:
            feature = pending.pop()
            if feature not in enabled:
                enabled.add(feature)
                pending.extend(item for item in features.get(feature, []) if item in features)
        if enabled & (DOMAINS - {domain}):
            errors.append(f"Cargo.toml: {domain} must not enable the other domain")

    paths_to_check = set()
    for kind, directory in (("test", "tests"), ("example", "examples"), ("bin", "src/bin")):
        listed = {}
        names = set()
        for target in manifest.get(kind, []):
            name = target.get("name", "")
            fallback = f"{directory}/{name}.rs"
            path = Path(target.get("path", fallback))
            if name in names or path in listed:
                errors.append(f"Cargo.toml: duplicate {kind} target {name} / {path}")
            names.add(name)
            listed[path] = target
            if not (root / path).is_file():
                errors.append(f"Cargo.toml: {kind} target missing on disk: {path}")
            expected = domain_for(path)
            required = target.get("required-features", [])
            needed = sorted(DOMAINS) if expected == "presentation entrypoint" else [] if expected == "core" else [expected]
            if sorted(required) != needed:
                errors.append(f"Cargo.toml: {path} required-features must be {needed}")
            paths_to_check.add(path)
        for actual in (root / directory).glob("*.rs"):
            relative = actual.relative_to(root)
            if relative not in listed:
                errors.append(f"Cargo.toml: unregistered {kind} target {relative}")
        if kind == "bin":
            for binary, feature in (("borrow-fighters", "fighting"), ("borrow-adventure", "adventure")):
                if not any(target.get("name") == binary and target.get("required-features") == [feature] for target in listed.values()):
                    errors.append(f"Cargo.toml: missing {binary} binary gated by {feature}")
            if not any(target.get("name") == "borrow-story" and path.as_posix() == STORY_BINARY and sorted(target.get("required-features", [])) == sorted(DOMAINS) for path, target in listed.items()):
                errors.append(f"Cargo.toml: missing borrow-story binary at {STORY_BINARY} gated by both adventure and fighting")

    lib = root / "src/lib.rs"
    if lib.is_file():
        errors.extend(check_lib(lib.read_text()))
    else:
        errors.append("src/lib.rs: missing library entrypoint")
    paths_to_check.update(path.relative_to(root) for path in (root / "src").rglob("*.rs"))
    for path in sorted(paths_to_check):
        if (root / path).is_file():
            errors.extend(check_source((root / path).read_text(), path, domain_for(path)))
    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    errors = check_repository(args.root.resolve())
    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1
    print("domain boundaries and explicit Cargo targets ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

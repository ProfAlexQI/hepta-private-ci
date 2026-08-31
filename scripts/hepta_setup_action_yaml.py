"""Duplicate-safe parser for the repository composite-action YAML subset."""
from __future__ import annotations

import json
import re


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


class Entry:
    def __init__(self, key: str, value: str, line: int, child: int | None = None):
        self.key, self.value, self.line, self.child = key, value, line, child


class Scope:
    def __init__(self) -> None:
        self.entries: dict[str, Entry] = {}
        self.items: list[int] = []


class YamlIndex:
    def __init__(self, scopes: dict[int, Scope]):
        self.scopes, self.root = scopes, 0

    def entry(self, scope: int, key: str) -> Entry:
        require(key in self.scopes[scope].entries, f"YAML mapping lacks required key {key!r}")
        return self.scopes[scope].entries[key]

    def child(self, scope: int, key: str) -> int:
        child = self.entry(scope, key).child
        require(child is not None, f"YAML key {key!r} must own a mapping")
        return child


def _comment(value: str) -> str:
    quote = None
    escaped = False
    for i, char in enumerate(value):
        if quote == '"':
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                quote = None
        elif quote == "'":
            if char == "'" and not (i + 1 < len(value) and value[i + 1] == "'"):
                quote = None
        elif char in {"'", '"'}:
            quote = char
        elif char == "#" and (i == 0 or value[i - 1].isspace()):
            return value[:i].rstrip()
    return value.rstrip()


def _key(token: str, line: int) -> str:
    token = token.strip()
    require(bool(token), f"empty YAML key on line {line}")
    if token.startswith('"'):
        try:
            value = json.loads(token)
        except json.JSONDecodeError as error:
            fail(f"invalid quoted YAML key on line {line}: {error}")
        require(isinstance(value, str), f"non-string YAML key on line {line}")
        return value
    if token.startswith("'"):
        require(token.endswith("'") and len(token) > 1, f"unterminated YAML key on line {line}")
        return token[1:-1].replace("''", "'")
    require(not any(c in token for c in "[]{}&,*!|>@`"), f"unsupported YAML key on line {line}")
    return token


def _mapping(text: str, line: int) -> tuple[str, str]:
    quote = None
    escaped = False
    i = 0
    while i < len(text):
        char = text[i]
        if quote == '"':
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                quote = None
        elif quote == "'":
            if char == "'":
                if i + 1 < len(text) and text[i + 1] == "'":
                    i += 1
                else:
                    quote = None
        elif char in {"'", '"'}:
            quote = char
        elif char == ":" and (i + 1 == len(text) or text[i + 1].isspace()):
            key, value = _key(text[:i], line), _comment(text[i + 1 :].strip())
            require(key != "<<", f"YAML merge keys are forbidden on line {line}")
            require("&" not in value and not value.startswith("*"), f"YAML anchors/aliases are forbidden on line {line}")
            return key, value
        i += 1
    fail(f"unsupported YAML syntax on line {line}: {text!r}")
    raise AssertionError


def parse_yaml_index(text: str) -> YamlIndex:
    scopes: dict[int, Scope] = {0: Scope()}
    stack: list[tuple[int, int]] = [(-1, 0)]
    next_scope, block_indent = 1, None

    def new_scope() -> int:
        nonlocal next_scope
        value = next_scope
        next_scope += 1
        scopes[value] = Scope()
        return value

    def add(scope: int, key: str, value: str, line: int, indent: int) -> None:
        require(key not in scopes[scope].entries, f"duplicate YAML key {key!r} in one mapping on line {line}")
        child = new_scope() if value == "" else None
        scopes[scope].entries[key] = Entry(key, value, line, child)
        if child is not None:
            stack.append((indent, child))

    for line_no, raw in enumerate(text.splitlines(), 1):
        prefix = raw[: len(raw) - len(raw.lstrip())]
        require("\t" not in prefix, f"tabs are forbidden in YAML indentation on line {line_no}")
        if not raw.strip() or raw.lstrip().startswith("#"):
            continue
        indent = len(raw) - len(raw.lstrip(" "))
        stripped = raw[indent:]
        if block_indent is not None:
            if indent > block_indent:
                continue
            block_indent = None
        require(stripped not in {"---", "..."}, f"multiple YAML documents are forbidden on line {line_no}")
        while len(stack) > 1 and stack[-1][0] >= indent:
            stack.pop()
        parent = stack[-1][1]
        if stripped == "-" or stripped.startswith("- "):
            item = new_scope()
            scopes[parent].items.append(item)
            stack.append((indent, item))
            remainder = stripped[1:].lstrip()
            require(bool(remainder), f"empty YAML sequence item on line {line_no}")
            key, value = _mapping(remainder, line_no)
            logical = indent + 2
            add(item, key, value, line_no, logical)
            if re.fullmatch(r"[|>][+-]?[1-9]?", value):
                block_indent = logical
            continue
        key, value = _mapping(stripped, line_no)
        add(parent, key, value, line_no, indent)
        if re.fullmatch(r"[|>][+-]?[1-9]?", value):
            block_indent = indent
    require(bool(scopes[0].entries), "YAML document is empty")
    return YamlIndex(scopes)

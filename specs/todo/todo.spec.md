---
module: todo
version: 1
status: active
files:
  - src/main.rs

db_tables: []
depends_on: []
---

# Todo

## Purpose

Scan source trees for actionable TODO-style comments and expose both human-readable and deterministic JSON reports for local review or CI enforcement.

## Public API

| Surface | Behavior |
|---------|----------|
| Todo command | Scan a directory for TODO and FIXME markers. |
| Extended-marker option | Include HACK and XXX markers. |
| Result-limit option | Cap the ordered result set at N findings. |
| JSON-output option | Emit the versioned Fledge JSON envelope. |
| Blocking option | Exit non-zero when at least one selected marker is found. |

## Invariants

1. Only files with the committed source-extension allowlist are scanned.
2. Repository metadata, dependencies, build output, caches, virtual environments, and dotted directories are skipped.
3. Markers match whole identifier boundaries so names such as `MY_TODO_LIST` do not produce findings.
4. Default scans include TODO and FIXME; `--all` adds HACK and XXX without removing defaults.
5. JSON output retains schema version 1 and reports normalized slash-separated paths.
6. `--fail-on-todo` exits non-zero exactly when the final, limited result set is non-empty.
7. Unreadable directories and files are skipped without aborting the remaining scan.

## Behavioral Examples

```
Given a source tree containing a TODO comment and an identifier named MY_TODO_LIST
When the plugin scans with --json --fail-on-todo
Then it reports only the comment in a schema-version-1 envelope and exits non-zero
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Unreadable path | A directory or source file cannot be read | Skip it and continue scanning accessible paths. |
| Invalid limit | `--limit` is missing or not numeric | Keep the existing unlimited scan behavior. |
| Findings in blocking mode | The final result set is non-empty with `--fail-on-todo` | Emit the selected report and exit 1. |
| Non-source input | A file extension is outside the allowlist | Ignore it. |

## Dependencies

- Rust 2021 standard library
- Fledge plugin installation and command dispatch

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-07-12 | Document existing Todo scanner behavior for SpecSync 5 adoption. |

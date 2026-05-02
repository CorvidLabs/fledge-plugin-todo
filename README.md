# fledge-plugin-todo

A [fledge](https://github.com/CorvidLabs/fledge) plugin that scans your codebase for TODO/FIXME/HACK/XXX comments.

Built in Rust. Zero runtime dependencies.

## Install

```bash
fledge plugins install CorvidLabs/fledge-plugin-todo
```

## Usage

```bash
fledge todo                          # scan current directory (TODO + FIXME)
fledge todo src/                     # scan a specific directory
fledge todo --all                    # also include HACK and XXX
fledge todo --limit 20               # cap results
fledge todo --json                   # machine-readable
fledge todo --fail-on-todo           # exit 1 if any are found (CI gate)
```

## JSON output

```json
{
  "schema_version": 1,
  "action": "todo",
  "root": ".",
  "tags_searched": ["TODO", "FIXME"],
  "count": 1,
  "matches": [
    {
      "file": "src/main.rs",
      "line": 42,
      "tag": "TODO",
      "text": "handle the empty case"
    }
  ]
}
```

## Use in lanes

```toml
[lanes.audit]
description = "Project health audit"
fail_fast = false
steps = [
  "lint",
  "test",
  { run = "fledge todo --json | jq '.count'" },
]

[lanes.no-todos]
description = "Block PRs that introduce new TODOs"
steps = [
  { run = "fledge todo --fail-on-todo" }
]
```

## File types

Scans common source extensions:

```
rs  py  js  jsx  ts  tsx  go  rb  java  kt  swift
c   cpp h   hpp  cs  php sh  bash zsh   fish
lua ex  exs erl  clj scala
```

Skips: `.git`, `node_modules`, `target`, `.build`, `build`, `dist`, `vendor`, `__pycache__`, `.venv`, `venv`, `.next`, `.nuxt`, and any directory whose name starts with a dot.

## Identifier-aware matching

The marker has to be a whole-word match. `MY_TODO_LIST` won't trigger; `// TODO: handle this` will. This avoids false positives on identifiers that happen to contain the marker letters.

## Build

A pre-built binary ships at `bin/fledge-todo`. If `cargo` is on PATH at install time, the build hook recompiles from source for the host platform.

## Attribution

Originally written as a Rust reference plugin by [`corvid-agent/fledge-plugin-todo`](https://github.com/corvid-agent/fledge-plugin-todo). This fork adds source-file filtering, the fledge JSON envelope shape, identifier-aware matching, and additional flags (`--all`, `--limit`, `--fail-on-todo`).

## License

MIT

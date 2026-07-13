---
spec: todo.spec.md
---

## User Stories

- As a developer, I want actionable TODO-style comments without false positives from identifiers or generated content.
- As a CI owner, I want deterministic JSON and a blocking exit mode.

## Acceptance Criteria

### REQ-todo-001

The default scan SHALL find whole-word TODO and FIXME markers in allowlisted source files.

### REQ-todo-002

The `--all` option SHALL additionally find whole-word HACK and XXX markers.

### REQ-todo-003

The scanner SHALL exclude configured build, dependency, cache, repository-metadata, virtual-environment, and dotted directories.

### REQ-todo-004

The `--limit N` option SHALL truncate the result set to at most N findings before output and exit-status evaluation.

### REQ-todo-005

JSON mode SHALL emit schema version 1 with the action, root, searched markers, count, and normalized match records.

Acceptance Criteria
- The deterministic JSON smoke parses the envelope, checks every required top-level key, and checks the required keys on every emitted match record.

### REQ-todo-006

Blocking mode SHALL exit 1 when the final result set is non-empty and 0 when it is empty.

Acceptance Criteria
- The deterministic blocking smoke checks exit 1 for a temporary marker fixture and exit 0 after replacing it with a clean fixture.

### REQ-todo-007

Unreadable and unsupported paths SHALL be skipped without preventing accessible source files from being scanned.

## Constraints

- Scanning is deliberately lexical and does not parse language grammars or distinguish comments from string literals.
- Marker and source-extension sets are compiled into the binary.

## Out of Scope

- Editing, assigning, aging, or synchronizing discovered work items.
- Parsing language-specific abstract syntax trees.

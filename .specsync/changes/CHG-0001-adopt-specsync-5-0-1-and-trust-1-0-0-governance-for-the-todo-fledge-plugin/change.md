---
id: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-todo-fledge-plugin
state: draft
type: migration
base_commit: 15cfa27c3bdffd8f2b6b33ecb3bbed7e88f088fc
---

# Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for the Todo Fledge plugin

## Intent

Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for the Todo Fledge plugin

## Affected Canonical Specs

- `todo`

## Acceptance Criteria

- Strict SpecSync validation passes at 100% coverage; Claude
- Cursor
- Codex
- and Gemini integrations report installed; Trust doctor and the native verify lane pass; ShellCheck
- Rust formatting
- clippy
- tests
- release build
- manifest validation
- JSON smoke
- and blocking-mode smoke pass.

## No-spec Rationale

Not applicable

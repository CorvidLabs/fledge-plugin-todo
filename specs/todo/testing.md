---
spec: todo.spec.md
---

## Test Plan

### Native Validation

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo build --release`
- `shellcheck bin/build.sh`

### Behavioral Smoke Validation

- Parse `--json` output and require schema version 1.
- Verify `--fail-on-todo` exits non-zero when scanning a temporary source file containing a TODO.
- Verify the plugin manifest and prebuilt-binary path remain valid.

### Governance Validation

- Run SpecSync strict validation at 100% file and LOC coverage.
- Run Trust doctor and the unified native verification lane.

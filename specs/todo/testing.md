---
spec: todo.spec.md
---

## Test Plan

### Native Build Validation

- `cargo clippy -- -D warnings`
- `cargo test` as a compile-time test-target check; the repository currently defines zero Rust unit tests.
- `cargo build --release`
- `shellcheck bin/build.sh`

### Behavioral Smoke Validation

- Parse `--json` output and require the versioned envelope and match-record keys.
- Verify `--fail-on-todo` exits 1 for a temporary source file containing a marker and 0 for a clean fixture.
- Verify the plugin manifest and prebuilt-binary path remain valid.

### Governance Validation

- Run SpecSync strict validation at 100% file and LOC coverage.
- Run Trust doctor and the unified native verification lane.

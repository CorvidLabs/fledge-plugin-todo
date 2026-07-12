---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-todo-fledge-plugin
artifact: testing
---

# Testing

- Run Rust formatting, clippy with warnings denied, tests, and release build.
- Run ShellCheck on the build hook.
- Parse JSON smoke output and verify schema version and action.
- Confirm blocking mode exits 1 when findings exist.
- Require SpecSync strict validation at 100% coverage and all four integrations installed.
- Run fledge trust doctor and fledge trust verify.
- Confirm hosted CI and the trust job on the pull request.


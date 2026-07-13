---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-todo-fledge-plugin
artifact: testing
---

# Testing

- Run Clippy with warnings denied, compile the zero-case Rust test target, and build the release binary.
- Run ShellCheck on the build hook.
- Parse JSON smoke output and verify the versioned envelope and match-record keys.
- Confirm blocking mode exits 1 for a deterministic marker fixture and 0 for a clean fixture.
- Require SpecSync strict validation at 100% coverage and all four integrations installed.
- Run fledge trust doctor and fledge trust verify.
- Confirm hosted CI and the trust job on the pull request.

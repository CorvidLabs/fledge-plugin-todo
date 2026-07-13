---
spec: todo.spec.md
---

## Context

Todo provides a small, dependency-free source scanner that can be used interactively or as a deterministic Fledge lane gate.

## Related Modules

- Fledge owns plugin installation and dispatch.
- The prebuilt binary and build hook package the Rust implementation for plugin consumers.

## Design Decisions

- Use an explicit source-extension allowlist and directory denylist to avoid binaries, generated output, and vendored dependencies.
- Match identifier boundaries to reduce false positives without introducing parser dependencies.
- Apply the result limit before evaluating blocking mode so output and exit status describe the same result set.

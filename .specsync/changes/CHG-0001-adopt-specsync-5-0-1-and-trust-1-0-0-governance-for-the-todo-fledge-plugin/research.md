---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-todo-fledge-plugin
artifact: research
---

# Research

- The implementation is a dependency-free Rust 2021 binary with a shell build hook and committed prebuilt executable.
- Existing CI runs ShellCheck, release build, JSON parsing, and blocking-mode smoke validation.
- Pages publishes static documentation independently on main.
- The plugin scans an explicit source allowlist and skips generated, dependency, cache, and dotted directories.
- SpecSync can measure the single Rust source file at 100% file and LOC coverage.


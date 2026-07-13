---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-todo-fledge-plugin
artifact: context
---

# Context

The Todo repository has Rust build and behavioral smoke CI plus an independent Pages deployment, but no canonical SpecSync lifecycle or unified Trust gate. The migration must preserve scanning semantics, packaging, ShellCheck, smoke tests, and Pages ownership.


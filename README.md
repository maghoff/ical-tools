Goal
====
A suite of Rust crates to enable easy parsing and generating of the ical format,
so easy that it makes ical that much easier to reach for where it's relevant.

Crates
======
- `ical-syntax`: Primitives for working with the ical syntax only, with no
    regard to the semantics
- `ical-derive`: Macros for creating ical parsers and generators for `struct`s,
    enforcing spec correctness

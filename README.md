Rume
===
Rume is a fork of Rime, with the intention to use Rust code and add personal
extensions. The idea is to also keep in sync with the original Rime upstream.
All the credit goes to the original Rime authors.

## Design decisions

- Use Rust code instead of C++ code
- Have a similar public API (at least for now), but not compatible
- Remove some of the custom logic in the config section
- Minimize generics (except in simple cases, when no need to downcast), but use traits and macros to reduce repetition

Rume
===
Rume is a fork of Rime, with the intention to use Rust code and add personal
extensions. The idea is to also keep in sync with the original Rime upstream.
All the credit goes to the original Rime authors.

## Design decisions

- Use Rust code instead of C++ code
- Expose a different C API, but have support for same Plum configuration, and same tool scripts
- Write idiomatic Rust, don't base the architecture or naming on the original library
- Initially write simple and repetitive Rust code, use as few generics as possible

rume-extension:
	cargo clippy --all-targets --all-features -- -D warnings && \
	cargo test --release --all-targets && \
	cargo build --release --all-targets && \
	cbindgen --config cbindgen.toml --crate rume_extension --output rume_extension.h
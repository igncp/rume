# a minimal build of third party libraries for static linking

rime_root = $(CURDIR)
src_dir = $(rime_root)/deps

ifndef NOPARALLEL
export MAKEFLAGS+=" -j$$(( $$(nproc 2>/dev/null || getconf _NPROCESSORS_ONLN 2>/dev/null || getconf NPROCESSORS_ONLN 2>/dev/null || echo 8) + 1)) "
endif

build ?= build
prefix ?= $(rime_root)

rime_deps = rume_extension

.PHONY: all clean clean-dist clean-src $(rime_deps)

all: $(rime_deps)

clean: clean-src clean-dist

clean-dist:
	git rev-parse --is-inside-work-tree > /dev/null && \
	find $(prefix)/bin $(prefix)/include $(prefix)/lib $(prefix)/share \
	-depth -maxdepth 1 \
	-exec bash -c 'git ls-files --error-unmatch "$$0" > /dev/null 2>&1 || rm -rv "$$0"' {} \; || true
	rmdir $(prefix) 2> /dev/null || true

# note: this won't clean output files under bin/, include/, lib/ and share/.
clean-src:
	for dep in $(rime_deps); do \
		rm -r $(src_dir)/$${dep}/$(build) || true; \
	done \
	&& rm -r $(src_dir)/rume_extension/target || true


rume_extension:
	mkdir -p $(prefix)/lib $(prefix)/include
	cd $(src_dir)/rume_extension; \
	cargo clippy --all-targets --all-features -- -D warnings && \
	cargo test && \
	cargo build --release && \
	cp target/release/librume_extension.a $(prefix)/lib/ && \
	cbindgen --config cbindgen.toml --crate rume_extension --output rume_extension.h && \
	cp rume_extension.h $(prefix)/include/;
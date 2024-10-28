GIT_ROOT := $(shell git rev-parse --show-toplevel)

all:
	cargo build --release
	cp $(GIT_ROOT)/target/release/libtoml_lua.so $(GIT_ROOT)/toml.so
	cargo clean

clean:
	rm $(GIT_ROOT)/toml.so

.PHONY: all clean

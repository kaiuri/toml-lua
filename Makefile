ROOT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
PKG_NAME := $(subst -,_,$(shell cargo read-manifest | sed -n 's/{"name":"\([^"]\+\)".*/\1/p'))
ARTIFACT_NAME := $(PKG_NAME).so

.PHONY: all clean

all:
	@cargo build --release
	cp $(ROOT_DIR)/target/release/lib$(ARTIFACT_NAME) $(ROOT_DIR)/$(ARTIFACT_NAME)
	cargo clean

clean:
	rm $(ROOT_DIR)/$(ARTIFACT_NAME)


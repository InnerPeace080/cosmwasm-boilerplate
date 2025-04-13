
# mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
# current_dir := $(notdir $(patsubst %/,%,$(dir $(mkfile_path))))

ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

VERSION:=$(shell grep '^version =' Cargo.toml | head -1 | cut -d '"' -f2)

build:
	cargo build --target wasm32-unknown-unknown --release
	@cp $(ROOT_DIR)/target/wasm32-unknown-unknown/release/cosmwasm-boilerplate.wasm $(ROOT_DIR)/artifacts/cosmwasm-boilerplate-$(VERSION).wasm
	ls -alh $(ROOT_DIR)/target/wasm32-unknown-unknown/release

build-optimizer:
	docker run --rm -v "$(ROOT_DIR)":/code \
	--mount type=volume,source="$(shell basename "$(ROOT_DIR)")_cache",target=/target \
	--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
	cosmwasm/optimizer:0.16.0
	@cp $(ROOT_DIR)/artifacts/cosmwasm-boilerplate.wasm $(ROOT_DIR)/artifacts/cosmwasm-boilerplate-$(VERSION).wasm
	ls -alh $(ROOT_DIR)/artifacts

check: build-optimizer
	cosmwasm-check $(ROOT_DIR)/artifacts/cosmwasm-boilerplate.wasm
	cosmwasm-check $(ROOT_DIR)/artifacts/cosmwasm-boilerplate-$(VERSION).wasm

test:
	RUSTC_BOOTSTRAP=1 cargo test --lib -- --nocapture

include build/common.mk

render:
	cargo run -p render-test

ui:
	cargo run -p ui-test
	cargo run -p ui-test --release

uui:
	cargo run -p ui-test --release -- --stop-on-failure --headless

all:
	order
	make wasm
	make ios
	make ui
	make render

fix:
	cargo fix --allow-dirty --allow-staged --all

.PHONY: bench
bench:
	cargo run -p bench --release

mobile:
	cargo install test-mobile
	test-mobile --path=../test-mobile/mobile-template

OS := $(shell uname)

build-ios:
ifeq ($(OS), Darwin)
	env CFLAGS="" SDKROOT="" cargo lipo -p test-game
else
	@echo " build-ios can only be run on macOS."
endif

ios-debug:
	cargo lipo -p test-game
	rm -f ./target/universal/release/libtest_game.a
	cp ./target/universal/debug/libtest_game.a ./target/universal/release/libtest_game.a

fix-lint:
	cargo clippy --fix --allow-dirty --allow-staged --workspace --all-targets

.PHONY: ci
ci:
	typos
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets -- -D warnings
	cargo machete

lint:
	cargo clippy --workspace --all-targets -- -D warnings

serve:
	rustup target add wasm32-unknown-unknown
	cargo install --locked trunk
	cd ./test-game && trunk serve --features webgl --address 0.0.0.0 --port 44800

serve-release:
	rustup target add wasm32-unknown-unknown
	cargo install --locked trunk
	cd ./test-game && trunk serve --features webgl --release --address 0.0.0.0 --port 44800

serve-size:
	rustup target add wasm32-unknown-unknown
	cargo install --locked trunk
	cd ./test-game && trunk serve --features webgl --cargo-profile=size --address 0.0.0.0 --port 44800

wasm:
	rustup target add wasm32-unknown-unknown
	cargo install --locked trunk
	cd ./test-game && trunk build

.PHONY: import
import:
	cargo fix --allow-dirty --allow-staged --all --all-targets

.PHONY: mobile

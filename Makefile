
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

CLIPPY_FLAGS = -- \
      \
      -W clippy::all \
      -W clippy::pedantic \
      \
      -A clippy::module_name_repetitions \
      -A clippy::explicit_deref_methods \
      -A clippy::missing_panics_doc \
      -A clippy::missing_errors_doc \
      -A clippy::missing_safety_doc \
      -A clippy::format_push_string \
      -A clippy::new_without_default \
      -A clippy::must_use_candidate \
      -A clippy::module_inception \
      -A clippy::needless_pass_by_value \
      -A clippy::unnecessary_box_returns \
      -A clippy::return_self_not_must_use \
      -A clippy::struct_field_names \
      -A clippy::manual_assert \
      -A dead_code

fix-lint:
	cargo clippy --fix --allow-dirty --allow-staged $(CLIPPY_FLAGS)

lint:
	cargo clippy $(CLIPPY_FLAGS) \
      \
      -D warnings

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

enc:
	sops -e secrets/decrypted/test-game.yml > secrets/test-game.enc.yml
	rm -rf secrets/decrypted

decr:
	mkdir secrets/decrypted
	sops -d secrets/test-game.enc.yml > secrets/decrypted/test-game.yml


.PHONY: import
import:
	cargo fix --allow-dirty --allow-staged --all --all-targets

.PHONY: mobile

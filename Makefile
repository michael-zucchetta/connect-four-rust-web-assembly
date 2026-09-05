
.PHONY: build-local build-web build-production serve-web run run-console

build-local:
	cargo fix --lib -p connect_four --allow-dirty

build-web:
	wasm-pack build --target web --out-dir pkg

build-production:
	RUSTFLAGS="-C opt-level=3" wasm-pack build --release --target web --out-dir pkg
	cp pkg/connect_four.js demo/connect_four.js
	cp pkg/connect_four_bg.wasm demo/connect_four_bg.wasm
	cp pkg/connect_four_bg.wasm demo/connect_four.wasm

serve-web:
	python3 -m http.server 8000

run:
	wasm-pack build --target web --out-dir pkg
	python3 -m http.server 8000

run-console:
	cargo run

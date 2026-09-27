
.PHONY: build-local build-web build-production build-production-white build-production-terminal copy-production-artifacts serve run run-console capture-readme-screenshot build-full

build-local:
	cargo fix --lib -p connect_four --allow-dirty

build-web:
	wasm-pack build --target web --out-dir pkg

copy-production-artifacts:
	cp pkg/connect_four.js demo/connect_four.js
	cp pkg/connect_four_bg.wasm demo/connect_four_bg.wasm
	cp pkg/connect_four_bg.wasm demo/connect_four.wasm

build-production: build-production-white

build-production-white:
	RUSTFLAGS="-C opt-level=3" wasm-pack build --release --target web --out-dir pkg
	$(MAKE) copy-production-artifacts
	sed -i 's/data-production-theme="[^"]*"/data-production-theme="white"/' index.html

build-production-terminal:
	RUSTFLAGS="-C opt-level=3" wasm-pack build --release --target web --out-dir pkg
	$(MAKE) copy-production-artifacts
	sed -i 's/data-production-theme="[^"]*"/data-production-theme="terminal"/' index.html

capture-readme-screenshot:
	python3 scripts/capture-readme-screenshot.py

build-full: build-production-terminal capture-readme-screenshot

serve:
	python3 -m http.server 8000

run:
	wasm-pack build --target web --out-dir pkg
	python3 -m http.server 8000

run-console:
	cargo run

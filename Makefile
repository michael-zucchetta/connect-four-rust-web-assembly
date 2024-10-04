
.PHONY: build-local build-web serve-web run run-console verify-firefox

build-local:
	cargo fix --lib -p connect_four --allow-dirty

build-web:
	wasm-pack build --target web --out-dir pkg

serve-web:
	python3 -m http.server 8000

run:
	wasm-pack build --target web --out-dir pkg
	python3 -m http.server 8000

run-console:
	cargo run

verify-firefox:
	python3 scripts/verify_firefox.py

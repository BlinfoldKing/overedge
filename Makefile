.PHONY: build

build:
	@wasm-pack build --target web --out-name wam --out-dir ./static/js

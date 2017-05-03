build:
	cargo build --target asmjs-unknown-emscripten --release
	sudo cp target/asmjs-unknown-emscripten/release/spring-crabs.js .

web:
	cargo run --target "wasm32-unknown-unknown"

r:
	clear && cargo run --target "x86_64-unknown-linux-gnu"
b:
	clear && cargo build --target "x86_64-unknown-linux-gnu"

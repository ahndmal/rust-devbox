#prepare:
#	rustup update
#    rustup update nightly
#    rustup target add wasm32-unknown-unknown --toolchain nightly
run:
	cargo run --color=always --package rust-features --bin rust-features
run_nightly:
	cargo +nightly run
release:
	cargo +nightly run --release
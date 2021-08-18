all: cargo-testall

cargo-example-minmax-unsigned:
	cargo run --example 01_minmax -- --test-threads 1 --nocapture 

cargo-example-json:
	cargo run --example 02_json -- --test-threads 1 --nocapture 

cargo-testall:
	cargo test -- --test-threads 1 --nocapture 

cargo-deny-list:
	cargo deny list

cargo-deny-check:
	 cargo deny check

fix:
	cargo clean && rm -rf Cargo.lock 

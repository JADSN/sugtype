all: cargo-example-minmax

cargo-example-minmax:
	 cargo run --example 01_minmax

cargo-testall:
	cargo test --  --test-threads 1 --nocapture 

cargo-deny-list:
	cargo deny list

cargo-deny-check:
	 cargo deny check
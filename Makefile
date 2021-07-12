all: run

run:
	cargo run -p app

cargo-deny-list:
	cargo deny list

cargo-deny-check:
	 cargo deny check
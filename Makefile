show1:
	cargo run examples/test1.html examples/test1.css

show2:
	cargo run examples/rainbow.html examples/rainbow.css

build:
	cargo build

run:
	cargo run

test:
	cargo test


.PHONY: build run test show

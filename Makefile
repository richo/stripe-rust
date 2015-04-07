examples: bin/cards bin/charge bin/customers

build:
	cargo build

.PHONY: build examples

bin:
	mkdir -p bin

bin/%: example/%.rs build bin
	rustc -L target/debug -L target/debug/deps $< -o $@

RUST_FLAGS = -L ext/rust-http/build  -L ext/rust-openssl/build
client: src/main.rs
	rustc -o $@ $(RUST_FLAGS) $<

ext: ext/rust-http ext/rust-openssl ext-http

ext/rust-http:
	git submodule update --init

ext/rust-openssl:
	git submodule update --init

ext-http: ext-openssl ext/rust-http/Makefile
	cd ext/rust-http; make

ext-openssl: ext/rust-openssl/Makefile
	cd ext/rust-openssl; make

ext/rust-openssl/Makefile:
	cd ext/rust-openssl; ./configure

ext/rust-http/Makefile:
	cd ext/rust-http; ./configure

.PHONY: ext ext-http ext-openssl

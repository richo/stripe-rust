RUSTC = rustc
RUST_FLAGS = -L ext/rust-http/build  -L ext/rust-openssl/build
libstripe_so = build.libstripe.timestamp

all: $(libstripe_so)

client: src/main.rs $(libstripe_so)
	$(RUSTC) -o $@ $(RUST_FLAGS) -L build $<

$(libstripe_so): Makefile src/connection.rs src/lib.rs
	mkdir -p build/
	$(RUSTC) $(RUST_FLAGS) src/lib.rs --out-dir=build
	@touch $@

# {{{ Setup submodules
ext: ext/rust-http ext/rust-openssl ext-http

ext/rust-http/configure:
	git submodule update --init

ext/rust-openssl/configure:
	git submodule update --init

ext-http: ext-openssl ext/rust-http/Makefile
	cd ext/rust-http; make

ext-openssl: ext/rust-openssl/Makefile
	cd ext/rust-openssl; make

ext/rust-openssl/Makefile: ext/rust-openssl/configure
	cd ext/rust-openssl; ./configure

ext/rust-http/Makefile: ext/rust-http/configure
	cd ext/rust-http; ./configure

.PHONY: ext ext-http ext-openssl
# }}}

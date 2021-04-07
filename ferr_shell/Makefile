all: release

fmt:
	cargo fmt

clean: fmt
	cargo clean
	rm -f ferr_shell

release: fmt
	cargo build --release
	cp -f target/x86_64-ferros/release/ferr_shell ferr_shell
	cp -f ferr_shell ../../FerrOS/src/ferr_shell

.PHONY: all fmt clean

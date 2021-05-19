all: release

fmt:
	cargo fmt

clean: fmt
	cargo clean

release: fmt
	cargo build --release
	cp -f target/x86_64-ferros/release/neofetch ../../FerrOS/filesystem/bin/neofetch

.PHONY: all fmt doc clean
